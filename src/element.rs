//! One line of a `.dly` file
//!
//! Each line is comprised of a element type and up to 31 (one for each day of the month)
//! datapoints for value, measurement flag, quality flag and source flag.
//!
//! More information about the format can be found
//! under section III of ftp://ftp.ncdc.noaa.gov/pub/data/ghcn/daily/readme.txt

use std::str::FromStr;

pub struct DlyLine;

pub const TUPLE_OFFSET: usize = 8;

impl DlyLine {
    //
    // RAW
    //

    pub fn station_id_raw(line: &str) -> &str {
        return &line[0..11];
    }

    pub fn year_raw(line: &str) -> &str {
        return &line[11..][..4];
    }

    pub fn month_raw(line: &str) -> &str {
        return &line[15..][..2];
    }

    pub fn element_raw(line: &str) -> &str {
        return &line[17..][..4];
    }

    pub fn value_raw(line: &str, index: usize) -> &str {
        let base_start = 21;
        let start = base_start + (TUPLE_OFFSET * index);
        return &line[start..][..5];
    }

    pub fn measurement_flag_raw(line: &str, index: usize) -> &str {
        let base_start = 26;
        let start = base_start + (TUPLE_OFFSET * index);
        return &line[start..][..1];
    }

    pub fn quality_flag_raw(line: &str, index: usize) -> &str {
        let base_start = 27;
        let start = base_start + (TUPLE_OFFSET * index);
        return &line[start..][..1];
    }

    pub fn source_flag_raw(line: &str, index: usize) -> &str {
        let base_start = 28;
        let start = base_start + (TUPLE_OFFSET * index);
        return &line[start..][..1];
    }

    //
    // TYPED
    //

    pub fn element(line: &str) -> Element {
        return Element::from_str(Self::element_raw(line)).unwrap();
    }

    pub fn measurement(line: &str, index: usize) -> Measurement {
        return Measurement::from_str(Self::measurement_flag_raw(line, index)).unwrap();
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Element {
    /// PRCP = Precipitation (tenths of mm)
    Precipation,
    /// SNOW = Snowfall (mm)
    Snowfall,
	/// SNWD = Snow depth (mm)
    Snowdepth,
    /// TMAX = Maximum temperature (tenths of degrees C)
    MaxTemp,
    // TMIN = Minimum temperature (tenths of degrees C)
    MinTemp,
}

impl FromStr for Element {
    type Err = ();

    fn from_str(s: &str) -> Result<Element, ()> {
        match s {
            "PRCP" => Ok(Element::Precipation),
            "SNOW" => Ok(Element::Snowfall),
            "SNWD" => Ok(Element::Snowdepth),
            "TMAX" => Ok(Element::MaxTemp),
            "TMIN" => Ok(Element::MinTemp),
            _ => Err(()),
            // TODO: implement "Other" type for other 4 character codes
        }
    }
}

/// Different measurement methods.
///
/// There are ten possible values.
#[derive(Debug, Copy, Clone)]
pub enum Measurement {
    /// Blank = no measurement information applicable
    None,
    /// B = precipitation total formed from two 12-hour totals
    TwoTotals,
    /// D = precipitation total formed from four six-hour totals
    FourTotals,
    /// H = represents highest or lowest hourly temperature (TMAX or TMIN) or the average of hourly values (TAVG)
    Hourly,
    /// K = converted from knots
    ConvertedKnots,
    /// L = temperature appears to be lagged with respect to reported hour of observation
    Lagged,
    /// O = converted from oktas
    ConvertedOktas,
    /// P = identified as "missing presumed zero" in DSI 3200 and 3206
    MissingPresumedZero,
    /// T = trace of precipitation, snowfall, or snow depth
    Trace,
    /// W = converted from 16-point WBAN code (for wind direction)
    Converted16PointWBAN,
}

impl FromStr for Measurement {
    type Err = ();

    fn from_str(s: &str) -> Result<Measurement, ()> {
        println!("{:?}", s);
        match s {
            " " => Ok(Measurement::None),
            "B" => Ok(Measurement::TwoTotals),
            "D" => Ok(Measurement::FourTotals),
            "H" => Ok(Measurement::Hourly),
            "K" => Ok(Measurement::ConvertedKnots),
            "L" => Ok(Measurement::Lagged),
            "O" => Ok(Measurement::ConvertedOktas),
            "P" => Ok(Measurement::MissingPresumedZero),
            "T" => Ok(Measurement::Trace),
            "W" => Ok(Measurement::Converted16PointWBAN),
            _ => Err(()),
        }
    }
}

// Unimplmented sources:
//
//            Blank = No source (i.e., data value missing)
//            0     = U.S. Cooperative Summary of the Day (NCDC DSI-3200)
//            6     = CDMP Cooperative Summary of the Day (NCDC DSI-3206)
//            7     = U.S. Cooperative Summary of the Day -- Transmitted
// 	           via WxCoder3 (NCDC DSI-3207)
//            A     = U.S. Automated Surface Observing System (ASOS)
//                    real-time data (since January 1, 2006)
// 	   a     = Australian data from the Australian Bureau of Meteorology
//            B     = U.S. ASOS data for October 2000-December 2005 (NCDC
//                    DSI-3211)
// 	   b     = Belarus update
// 	   C     = Environment Canada
// 	   E     = European Climate Assessment and Dataset (Klein Tank
// 	           et al., 2002)
//            F     = U.S. Fort data
//            G     = Official Global Climate Observing System (GCOS) or
//                    other government-supplied data
//            H     = High Plains Regional Climate Center real-time data
//            I     = International collection (non U.S. data received through
// 	           personal contacts)
//            K     = U.S. Cooperative Summary of the Day data digitized from
// 	           paper observer forms (from 2011 to present)
//            M     = Monthly METAR Extract (additional ASOS data)
// 	   N     = Community Collaborative Rain, Hail,and Snow (CoCoRaHS)
// 	   Q     = Data from several African countries that had been
// 	           "quarantined", that is, withheld from public release
// 		   until permission was granted from the respective
// 	           meteorological services
//            R     = NCEI Reference Network Database (Climate Reference Network
// 	           and Regional Climate Reference Network)
// 	   r     = All-Russian Research Institute of Hydrometeorological
// 	           Information-World Data Center
//            S     = Global Summary of the Day (NCDC DSI-9618)
//                    NOTE: "S" values are derived from hourly synoptic reports
//                    exchanged on the Global Telecommunications System (GTS).
//                    Daily values derived in this fashion may differ significantly
//                    from "true" daily data, particularly for precipitation
//                    (i.e., use with caution).
// 	   s     = China Meteorological Administration/National Meteorological Information Center/
// 	           Climatic Data Center (http://cdc.cma.gov.cn)
//            T     = SNOwpack TELemtry (SNOTEL) data obtained from the U.S.
// 	           Department of Agriculture's Natural Resources Conservation Service
// 	   U     = Remote Automatic Weather Station (RAWS) data obtained
// 	           from the Western Regional Climate Center
// 	   u     = Ukraine update
// 	   W     = WBAN/ASOS Summary of the Day from NCDC's Integrated
// 	           Surface Data (ISD).
//            X     = U.S. First-Order Summary of the Day (NCDC DSI-3210)
// 	   Z     = Datzilla official additions or replacements
// 	   z     = Uzbekistan update

/// Different sources.
///
/// There are twenty nine possible values
/// (including blank, upper and lower case letters).
///
/// When data are available for the same time from more than one source,
/// the highest priority source is chosen according to the following
/// priority order (from highest to lowest):
///
/// Z,R,0,6,C,X,W,K,7,F,B,M,r,E,z,u,b,s,a,G,Q,I,A,N,T,U,H,S
#[derive(Debug, Copy, Clone)]
pub enum Source {

}
