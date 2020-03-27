/*
 * Traditional astrology for rust
 * ==============================
 *
 * Rust library by Stéphane (s.bressani@bluewin.ch)
 *
 * Using swissephem c library by Astrodienst AG
 * by Dieter Koch and Alois Treindl (https://www.astro.com/ftp/swisseph/)
 *
 * The source code is released under an CC License, which allows it to be used
 * also on commercial projects. This software uses the swiss ephemeris which is
 * licensed GPL.
 *
 * Therefore, if you want to use astro_compute_swisseph in your commercial
 * projects, you must adhere to the GPL license or buy a Swiss Ephemeris
 * commercial license.
 */
use crate::raw;
use crate::sweconst::Bodies;
use crate::swerust;
use std::ffi::{CStr, CString};

/*
 * 3. The functions swe_calc_ut() and swe_calc()
 *
 * Before calling one of these functions or any other Swiss Ephemeris function,
 * it is strongly recommended to call the function swe_set_ephe_path(). Even if
 * you don’t want to set an ephemeris path and use the Moshier ephemeris, it is
 * nevertheless recommended to call swe_set_ephe_path(NULL), because this
 * function makes important initializations. If you don’t do that, the Swiss
 * Ephemeris may work but the results may be not 100% consistent.
 */

/*
 * xx
 *
 * Ecliptic position               Equatorial position (SEFLG_EQUATORIAL)
 * Longitude                       right ascension
 * Latitude                        declination
 * Distance in AU                  distance in AU
 * Speed in longitude (deg/day)    speed in right ascension (deg/day)
 * Speed in latitude (deg/day)     speed in declination (deg/day)
 * Speed in distance (AU/day)      speed in distance (AU/day)
 */
#[derive(Debug)]
pub struct CalcUtResult {
    pub longitude: f64,
    pub latitude: f64,
    pub distance_au: f64,
    pub speed_longitude: f64,
    pub speed_latitude: f64,
    pub speed_distance_au: f64,
    pub status: i32,
    pub serr: String,
}

pub fn calc_ut(tjd_ut: f64, ipl: Bodies, iflag: i32) -> CalcUtResult {
    let mut xx: [f64; 6] = [0.0; 6];
    let mut serr = [0; 255];
    let result;
    result = unsafe {
        let p_xx = xx.as_mut_ptr();
        let p_serr = serr.as_mut_ptr();
        let status;
        if ipl == Bodies::SouthNode {
            status = raw::swe_calc_ut(
                tjd_ut,
                Bodies::TrueNode as i32,
                iflag,
                p_xx,
                p_serr,
            );
        } else {
            status = raw::swe_calc_ut(tjd_ut, ipl as i32, iflag, p_xx, p_serr);
        }
        let s_serr = CString::from(CStr::from_ptr(p_serr))
            .to_str()
            .unwrap()
            .to_string();
        if ipl == Bodies::SouthNode {
            xx[0] = xx[0] + 180.0;
            if xx[0] >= 360.0 {
                xx[0] = xx[0] - 360.0;
            }
        }
        CalcUtResult {
            longitude: xx[0],
            latitude: xx[1],
            distance_au: xx[2],
            speed_longitude: xx[3],
            speed_latitude: xx[4],
            speed_distance_au: xx[5],
            serr: s_serr,
            status: status,
        }
    };
    result
}

/// Fortuna Part
pub fn calc_ut_fp(
    tjd_ut: f64,
    geolat: f64,
    geolong: f64,
    hsys: char,
    iflag: i32,
) -> CalcUtResult {
    let ipl = Bodies::FortunaPart;
    let mut xx: [f64; 6] = [0.0; 6];
    let mut serr = [0; 255];
    let result = unsafe {
        let p_xx = xx.as_mut_ptr();
        let p_serr = serr.as_mut_ptr();
        let status = raw::swe_calc_ut(tjd_ut, ipl as i32, iflag, p_xx, p_serr);
        let s_serr = CString::from(CStr::from_ptr(p_serr))
            .to_str()
            .unwrap()
            .to_string();
        let mut xx_sun: [f64; 6] = [0.0; 6];
        let mut xx_moon: [f64; 6] = [0.0; 6];
        let p_xx_sun = xx_sun.as_mut_ptr();
        let p_serr_sun = serr.as_mut_ptr();
        let _status_sun = raw::swe_calc_ut(
            tjd_ut,
            Bodies::Sun as i32,
            iflag,
            p_xx_sun,
            p_serr_sun,
        );
        let p_xx_moon = xx_moon.as_mut_ptr();
        let p_serr_moon = serr.as_mut_ptr();
        let _status_moon = raw::swe_calc_ut(
            tjd_ut,
            Bodies::Moon as i32,
            iflag,
            p_xx_moon,
            p_serr_moon,
        );
        let _s_serr_sun = CString::from(CStr::from_ptr(p_serr_sun))
            .to_str()
            .unwrap()
            .to_string();
        let _s_serr_moon = CString::from(CStr::from_ptr(p_serr_moon))
            .to_str()
            .unwrap()
            .to_string();
        let result_houses =
            swerust::handler_swe14::houses(tjd_ut, geolat, geolong, hsys);
        let asc_lon = result_houses.cusps[0];
        let mc_lon = result_houses.cusps[0];
        let mc_lat = 0.0;
        let compute_sun = eq_coords(xx_sun[0], xx_sun[1]);
        let compute_mc = eq_coords(mc_lon, mc_lat);
        let sw_is_diurnal = is_above_horizon(
            compute_sun.0,
            compute_sun.1,
            compute_mc.0,
            compute_mc.1,
        );

        let mut lon = if sw_is_diurnal {
            asc_lon + xx_moon[0] - xx_sun[0]
        } else {
            asc_lon + xx_sun[0] - xx_moon[0]
        };
        let mut done = false;
        while !done {
            if lon >= 360.0 {
                lon = lon - 360.0;
            } else {
                done = true;
            }
        }
        CalcUtResult {
            longitude: lon,
            latitude: xx[1],
            distance_au: xx[2],
            speed_longitude: xx[3],
            speed_latitude: xx[4],
            speed_distance_au: xx[5],
            serr: s_serr,
            status: status,
        }
    };
    result
}

/// Converts from ecliptical to equatorial coordinates.
fn eq_coords(lon: f64, lat: f64) -> (f64, f64) {
    // Convert to radian
    let lambda = lon.to_radians();
    let beta = lat.to_radians();
    let epson = (23.44 as f64).to_radians(); // the earth inclinaison

    // Declinaison in radian
    let decl = (epson.sin() * lambda.sin() * beta.cos()
        + epson.cos() * beta.sin())
    .asin();

    // Equatorial distance
    let ed = (lambda.cos() * beta.cos() / decl.cos()).acos();

    // RA in radian
    let mut ra = if lon < 100.0 {
        ed
    } else {
        (360.0 as f64).to_radians() - ed
    };

    // Correctness of RA if longitude is close to 0° or 180° in a radius of 5°
    if (closest_distance(lon, 0.0)).abs() < 5.0
        || (closest_distance(lon, 180.0)).abs() < 5.0
    {
        let a = ra.sin() * decl.cos();
        let b =
            epson.cos() * lambda.sin() * beta.cos() - epson.sin() * beta.sin();
        if (a - b).abs() > 0.0003 {
            ra = (360.0 as f64).to_radians() - ra;
        }
    }
    (ra.to_degrees(), decl.to_degrees())
}

/// Boolean if is above horizon
/// Returns if an object's ra and decl is above the horizon at a specific
/// latitude, given the MC's right ascension.
///
/// This function checks if the equatorial distance from the object to the MC
/// is within its diurnal semi-arc.
fn is_above_horizon(ra: f64, decl: f64, mc_ra: f64, lat: f64) -> bool {
    let d_arc_tulpe = dnarcs(decl, lat);
    let dist = (closest_distance(mc_ra, ra)).abs();
    dist <= d_arc_tulpe.0 / 2.0 + 0.0003 // 1 arc-second
}

/// Returns the diurnal and nocturnal arcs of a point.
fn dnarcs(decl: f64, lat: f64) -> (f64, f64) {
    let d_arc = 180.0 + 2.0 * ascdiff(decl, lat);
    let n_arc = 360.0 - d_arc;
    (d_arc, n_arc)
}

/// Returns the Ascensional Difference of a point.
fn ascdiff(decl: f64, lat: f64) -> f64 {
    let delta = decl.to_radians();
    let phi = lat.to_radians();
    let ad = (delta.tan() * phi.tan()).asin();
    ad.to_degrees()
}

/// Closest distance between 2 point
fn closest_distance(angle1: f64, angle2: f64) -> f64 {
    znorm(angle2 - angle1)
}

/// Normalize angle between -180° and 180°
fn znorm(mut angle: f64) -> f64 {
    angle = angle % 360.0;
    if angle <= 180.0 {
        angle
    } else {
        angle - 180.0
    }
}
