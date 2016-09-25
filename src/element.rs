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

    pub fn station_id(line: &str) -> &str {
        Self::station_id_raw(line)
    }

    pub fn year(line: &str) -> Result<isize, ::std::num::ParseIntError> {
        Self::year_raw(line).parse()
    }

    pub fn month(line: &str) -> Result<isize, ::std::num::ParseIntError> {
        Self::month_raw(line).parse()
    }

    pub fn element(line: &str) -> Result<Element, ()> {
        Element::from_str(Self::element_raw(line))
    }

    pub fn value(line: &str, index: usize) -> Result<Option<isize>, ::std::num::ParseIntError> {
        let val: isize = try!(Self::value_raw(line, index).trim().parse());

        match val {
            -9999 => Ok(None),
            other => Ok(Some(other))
        }
    }

    pub fn measurement(line: &str, index: usize) -> Result<Measurement, ()> {
        Measurement::from_str(Self::measurement_flag_raw(line, index))
    }

    pub fn quality(line: &str, index: usize) -> Result<Quality, ()> {
        Quality::from_str(Self::quality_flag_raw(line, index))
    }

    pub fn source(line: &str, index: usize) -> Result<Source, ()> {
        Source::from_str(Self::source_flag_raw(line, index))
    }
}

/// Element types.
///
/// **Unimplmented values**:
///
/// - ACMC = Average cloudiness midnight to midnight from 30-second ceilometer data (percent)
/// - ACMH = Average cloudiness midnight to midnight from manual observations (percent)
/// - ACSC = Average cloudiness sunrise to sunset from 30-second ceilometer data (percent)
/// - ACSH = Average cloudiness sunrise to sunset from manual observations (percent)
/// - AWDR = Average daily wind direction (degrees)
/// - AWND = Average daily wind speed (tenths of meters per second)
/// - DAEV = Number of days included in the multiday evaporation total (MDEV)
/// - DAPR = Number of days included in the multiday precipiation total (MDPR)
/// - DASF = Number of days included in the multiday snowfall total (MDSF)
/// - DATN = Number of days included in the multiday minimum temperature (MDTN)
/// - DATX = Number of days included in the multiday maximum temperature (MDTX)
/// - DAWM = Number of days included in the multiday wind movement (MDWM)
/// - DWPR = Number of days with non-zero precipitation included in multiday precipitation total (MDPR)
/// - EVAP = Evaporation of water from evaporation pan (tenths of mm)
/// - FMTM = Time of fastest mile or fastest 1-minute wind (hours and minutes, i.e., HHMM)
/// - FRGB = Base of frozen ground layer (cm)
/// - FRGT = Top of frozen ground layer (cm)
/// - FRTH = Thickness of frozen ground layer (cm)
/// - GAHT = Difference between river and gauge height (cm)
/// - MDEV = Multiday evaporation total (tenths of mm; use with DAEV)
/// - MDPR = Multiday precipitation total (tenths of mm; use with DAPR and DWPR, if available)
/// - MDSF = Multiday snowfall total
/// - MDTN = Multiday minimum temperature (tenths of degrees C; use with DATN)
/// - MDTX = Multiday maximum temperature (tenths of degress C; use with DATX)
/// - MDWM = Multiday wind movement (km)
/// - MNPN = Daily minimum temperature of water in an evaporation pan (tenths of degrees C)
/// - MXPN = Daily maximum temperature of water in an evaporation pan (tenths of degrees C)
/// - PGTM = Peak gust time (hours and minutes, i.e., HHMM)
/// - PSUN = Daily percent of possible sunshine (percent)
/// - SN*# = Minimum soil temperature (tenths of degrees C)
///         where * corresponds to a code
///         for ground cover and # corresponds to a code for soil
///         depth.
///
///         Ground cover codes include the following:
///             0 = unknown
///             1 = grass
///             2 = fallow
///             3 = bare ground
///             4 = brome grass
///             5 = sod
///             6 = straw multch
///             7 = grass muck
///             8 = bare muck
///
///         Depth codes include the following:
///             1 = 5 cm
///             2 = 10 cm
///             3 = 20 cm
///             4 = 50 cm
///             5 = 100 cm
///             6 = 150 cm
///             7 = 180 cm
///
/// - SX*# = Maximum soil temperature (tenths of degrees C)
///         where * corresponds to a code for ground cover
///         and # corresponds to a code for soil depth.
///         See SN*# for ground cover and depth codes.
/// - THIC = Thickness of ice on water (tenths of mm)
/// - TOBS = Temperature at the time of observation (tenths of degrees C)
/// - TSUN = Daily total sunshine (minutes)
/// - WDF1 = Direction of fastest 1-minute wind (degrees)
/// - WDF2 = Direction of fastest 2-minute wind (degrees)
/// - WDF5 = Direction of fastest 5-second wind (degrees)
/// - WDFG = Direction of peak wind gust (degrees)
/// - WDFI = Direction of highest instantaneous wind (degrees)
/// - WDFM = Fastest mile wind direction (degrees)
/// - WDMV = 24-hour wind movement (km)
/// - WESD = Water equivalent of snow on the ground (tenths of mm)
/// - WESF = Water equivalent of snowfall (tenths of mm)
/// - WSF1 = Fastest 1-minute wind speed (tenths of meters per second)
/// - WSF2 = Fastest 2-minute wind speed (tenths of meters per second)
/// - WSF5 = Fastest 5-second wind speed (tenths of meters per second)
/// - WSFG = Peak gust wind speed (tenths of meters per second)
/// - WSFI = Highest instantaneous wind speed (tenths of meters per second)
/// - WSFM = Fastest mile wind speed (tenths of meters per second)
/// - WT** = Weather Type where ** has one of the following values:
///
///         01 = Fog, ice fog, or freezing fog (may include heavy fog)
///         02 = Heavy fog or heaving freezing fog (not always
///         distinquished from fog)
///         03 = Thunder
///         04 = Ice pellets, sleet, snow pellets, or small hail
///         05 = Hail (may include small hail)
///         06 = Glaze or rime
///         07 = Dust, volcanic ash, blowing dust, blowing sand, or
///         blowing obstruction
///         08 = Smoke or haze
///         09 = Blowing or drifting snow
///         10 = Tornado, waterspout, or funnel cloud
///         11 = High or damaging winds
///         12 = Blowing spray
///         13 = Mist
///         14 = Drizzle
///         15 = Freezing drizzle
///         16 = Rain (may include freezing rain, drizzle, and
///         freezing drizzle)
///         17 = Freezing rain
///         18 = Snow, snow pellets, snow grains, or ice crystals
///         19 = Unknown source of precipitation
///         21 = Ground fog
///         22 = Ice fog or freezing fog
///
/// - WV** = Weather in the Vicinity where ** has one of the following
/// values:
///
///         01 = Fog, ice fog, or freezing fog (may include heavy fog)
///         03 = Thunder
///         07 = Ash, dust, sand, or other blowing obstruction
///         18 = Snow or ice crystals
///         20 = Rain or snow shower
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
    /// TMIN = Minimum temperature (tenths of degrees C)
    MinTemp,
    /// TAVG = Average temperature (tenths of degrees C)
    ///         [Note that TAVG from source 'S' corresponds
    ///         to an average for the period ending at
    ///         2400 UTC rather than local midnight]
    AvgTemp,
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
            "TAVG" => Ok(Element::AvgTemp),
            _ => Err(()),
            // TODO: implement "Other" type for other 4 character codes
        }
    }
}

/// Measurement methods.
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

/// Quality indicators.
///
/// There are fourteen possible values.
#[derive(Debug, Copy, Clone)]
pub enum Quality {
    /// Blank = did not fail any quality assurance check
    None,
    /// D = failed duplicate check
    FailedDuplicate,
    /// G = failed gap check
    FailedGap,
    /// I = failed internal consistency check
    FailedInternalConsistency,
    /// K = failed streak/frequent-value check
    FailedStreak,
    /// L = failed check on length of multiday period
    FailedMultidayPeriod,
    /// M = failed megaconsistency check
    FailedMegaconsistency,
    /// N = failed naught check
    FailedNaught,
    /// O = failed climatological outlier check
    FailedClimatologicalOutlier,
    /// R = failed lagged range check
    FailedLaggedRange,
    /// S = failed spatial consistency check
    FailedSpatialConsistency,
    /// T = failed temporal consistency check
    FailedTemporalConsistency,
    /// W = temperature too warm for snow
    TooWarmForSnow,
    /// X = failed bounds check
    FailedBounds,
    /// Z = flagged as a result of an official Datzilla
    ///     investigation
    FlaggedDatzilla,
}

impl FromStr for Quality {
    type Err = ();

    fn from_str(s: &str) -> Result<Quality, ()> {
        match s {
            " " => Ok(Quality::None),
            "D" => Ok(Quality::FailedDuplicate),
            "G" => Ok(Quality::FailedGap),
            "I" => Ok(Quality::FailedInternalConsistency),
            "K" => Ok(Quality::FailedStreak),
            "L" => Ok(Quality::FailedMultidayPeriod),
            "M" => Ok(Quality::FailedMegaconsistency),
            "N" => Ok(Quality::FailedNaught),
            "O" => Ok(Quality::FailedClimatologicalOutlier),
            "R" => Ok(Quality::FailedLaggedRange),
            "S" => Ok(Quality::FailedSpatialConsistency),
            "T" => Ok(Quality::FailedTemporalConsistency),
            "W" => Ok(Quality::TooWarmForSnow),
            "X" => Ok(Quality::FailedBounds),
            "Z" => Ok(Quality::FlaggedDatzilla),
            _ => Err(()),
        }
    }
}

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
///
/// **Unimplmented values**:
///
/// 0 = U.S. Cooperative Summary of the Day (NCDC DSI-3200)
/// 6 = CDMP Cooperative Summary of the Day (NCDC DSI-3206)
/// 7 = U.S. Cooperative Summary of the Day -- Transmitted via WxCoder3 (NCDC DSI-3207)
/// A = U.S. Automated Surface Observing System (ASOS) real-time data (since January 1, 2006)
/// a = Australian data from the Australian Bureau of Meteorology
/// B = U.S. ASOS data for October 2000-December 2005 (NCDC DSI-3211)
/// b = Belarus update
/// C = Environment Canada
/// F = U.S. Fort data
/// G = Official Global Climate Observing System (GCOS) or other government-supplied data
/// H = High Plains Regional Climate Center real-time data
/// I = International collection (non U.S. data received through personal contacts)
/// K = U.S. Cooperative Summary of the Day data digitized from paper observer forms (from 2011 to present)
/// M = Monthly METAR Extract (additional ASOS data)
/// N = Community Collaborative Rain, Hail,and Snow (CoCoRaHS)
/// Q = Data from several African countries that had been "quarantined", that is, withheld from public release
///     until permission was granted from the respective meteorological services
/// R = NCEI Reference Network Database (Climate Reference Network and Regional Climate Reference Network)
/// r = All-Russian Research Institute of Hydrometeorological Information-World Data Center
/// s = China Meteorological Administration/National Meteorological Information Center/ Climatic Data Center (http:///cdc.cma.gov.cn)
/// T = SNOwpack TELemtry (SNOTEL) data obtained from the U.S. Department of Agriculture's Natural Resources Conservation Service
/// U = Remote Automatic Weather Station (RAWS) data obtained from the Western Regional Climate Center
/// u = Ukraine update
/// W = WBAN/ASOS Summary of the Day from NCDC's Integrated Surface Data (ISD).
/// X = U.S. First-Order Summary of the Day (NCDC DSI-3210)
/// Z = Datzilla official additions or replacements
/// z = Uzbekistan update
#[derive(Debug, Copy, Clone)]
pub enum Source {
    /// Blank = No source (i.e., data value missing)
    None,
    /// E = European Climate Assessment and Dataset (Klein Tank et al., 2002)
    ECAandD,
    /// S = Global Summary of the Day (NCDC DSI-9618)
    ///     NOTE: "S" values are derived from hourly synoptic reports
    ///     exchanged on the Global Telecommunications System (GTS).
    ///     Daily values derived in this fashion may differ significantly
    ///     from "true" daily data, particularly for precipitation
    ///     (i.e., use with caution).
    DSI9618,
}

impl FromStr for Source {
    type Err = ();

    fn from_str(s: &str) -> Result<Source, ()> {
        match s {
            " " => Ok(Source::None),
            "E" => Ok(Source::ECAandD),
            "S" => Ok(Source::DSI9618),
            _ => Err(()),
        }
    }
}
