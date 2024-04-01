use num_traits::FromPrimitive;
use num_derive::FromPrimitive;

// Instax protocol direction: to or from device
#[derive(Debug, FromPrimitive, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum Direction {
    TO = 0x4162,    // "Ab"
    FROM = 0x6142, // "aB"
}

// SID: Instax protocol opcodes
// u16 with modeCode, typeCode as big endian
#[derive(Debug, FromPrimitive, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum SID {
    UNKNOWN = 0xffff,
    SUPPORT_FUNCTION_AND_VERSION_INFO = 0x0000,
    DEVICE_INFO_SERVICE = 0x0001,
    SUPPORT_FUNCTION_INFO = 0x0002,
    IDENTIFY_INFORMATION = 0x0010,
    SHUT_DOWN = 0x0100,
    RESET = 0x0101,
    AUTO_SLEEP_SETTINGS = 0x0102,
    BLE_CONNECT = 0x0103,
    PRINT_IMAGE_DOWNLOAD_START = 0x1000,
    PRINT_IMAGE_DOWNLOAD_DATA = 0x1001,
    PRINT_IMAGE_DOWNLOAD_END = 0x1002,
    PRINT_IMAGE_DOWNLOAD_CANCEL = 0x1003,
    PRINT_IMAGE = 0x1080,
    REJECT_FILM_COVER = 0x1081,
    FW_DOWNLOAD_START = 0x2000,
    FW_DOWNLOAD_DATA = 0x2001,
    FW_DOWNLOAD_END = 0x2002,
    FW_UPGRADE_EXIT = 0x2003,
    FW_PROGRAM_INFO = 0x2010,
    FW_DATA_BACKUP = 0x2080,
    FW_UPDATE_REQUEST = 0x2081,
    XYZ_AXIS_INFO = 0x3000,
    LED_PATTERN_SETTINGS = 0x3001,
    AXIS_ACTION_SETTINGS = 0x3002,
    LED_PATTERN_SETTINGS_DOUBLE = 0x3003,
    POWER_ONOFF_LED_SETTING = 0x3004,
    AR_LED_VIBRARTION_SETTING = 0x3006,
    FUNCTION_BUTTON_SETTING = 0x3008,
    ADDITIONAL_PRINTER_INFO = 0x3010,
    PRINTER_HEAD_LIGHT_CORRECT_INFO = 0x3080,
    PRINTER_HEAD_LIGHT_CORRECT_SETTINGS = 0x3081,
    CAMERA_SETTINGS = 0x8000,
    CAMERA_SETTINGS_GET = 0x8001,
    ADDITIONAL_CAMERA_INFO = 0x8010,
    PARAMETER_RW = 0x8011,
    TIME_SETTING = 0x8012,
    URL_UPLOAD_INFO = 0x8100,
    URL_PICTURE_UPLOAD_START = 0x8101,
    URL_PICTURE_UPLOAD = 0x8102,
    URL_PICTURE_UPLOAD_END = 0x8103,
    URL_AUDIO_UPLOAD_START = 0x8104,
    URL_AUDIO_UPLOAD = 0x8105,
    URL_AUDIO_UPLOAD_END = 0x8106,
    URL_UPLOAD_ADDRESS = 0x8107,
    URL_UPLOAD_DATA_COMPLETE = 0x8108,
    LIVE_VIEW_START = 0x8200,
    LIVE_VIEW_RECEIVE = 0x8201,
    LIVE_VIEW_STOP = 0x8202,
    LIVE_VIEW_TAKE_PICTURE = 0x8210,
    POST_VIEW_UPLOAD_START = 0x8220,
    POST_VIEW_UPLOAD = 0x8221,
    POST_VIEW_UPLOAD_END = 0x8222,
    POST_VIEW_PRINT = 0x8230,
    FRAME_PICTURE_DOWNLOAD_START = 0x8300,
    FRAME_PICTURE_DOWNLOAD = 0x8301,
    FRAME_PICTURE_DOWNLOAD_END = 0x8302,
    FRAME_PICTURE_NAME_SETTING = 0x8303,
    FRAME_PICTURE_NAME_GET = 0x8304,
    CAMERA_LOG_SUBTOTAL_START = 0x8400,
    CAMERA_LOG_SUBTOTAL_DATA = 0x8401,
    CAMERA_LOG_SUBTOTAL_CLEAR = 0x8402,
    CAMERA_LOG_DATE_START = 0x8403,
    CAMERA_LOG_DATE_DATA = 0x8404,
    CAMERA_LOG_DATE_CLEAR = 0x8405,
    CAMERA_LOG_FILTER_START = 0x8406,
    CAMERA_LOG_FILTER_DATA = 0x8407,
    CAMERA_LOG_FILTER_CLEAR = 0x8408,
    CAMERA_LOG_RECORD_DATE_START = 0x8409,
    CAMERA_LOG_RECORD_DATE_DATA = 0x840a,
    CAMERA_LOG_RECORD_DATE_CLEAR = 0x840b,
    CHECK_CAMERA_STATUS = 0x8500,
    EXECUTE_CAMERA_COMMAND = 0x8501,
    SOUND_PLAY_STATUS = 0x8600,
    SOUND_DOWNLOAD_START = 0x8601,
    SOUND_DOWNLOAD = 0x8602,
    SOUND_DOWNLOAD_END = 0x8603,
    SOUND_DOWNLOAD_CANCEL = 0x8604,
    SOUND_PLAY_START = 0x8605,
    SOUND_PLAY_STOP = 0x8606,
    IMAGE_MANUAL_UPLOAD_INFO = 0x8700,
    IMAGE_MANUAL_UPLOAD_START = 0x8701,
    IMAGE_MANUAL_UPLOAD_DATA = 0x8702,
    IMAGE_MANUAL_UPLOAD_END = 0x8703,
    IMAGE_MANUAL_UPLOAD_CANCEL = 0x8704,
    IMAGE_AUTO_UPLOAD_INFO = 0x8800,
    IMAGE_AUTO_UPLOAD_START = 0x8801,
    IMAGE_AUTO_UPLOAD_DATA = 0x8802,
    IMAGE_AUTO_UPLOAD_END = 0x8803,
    IMAGE_AUTO_UPLOAD_CANCEL = 0x8804,
    IMAGE_AUTO_UPLOAD_COMPLETE = 0x8805,
    INTERVAL_RECORD_STATUS = 0x8900,
    INTERVAL_RECORD_START = 0x8901,
    INTERVAL_RECORD_STOP = 0x8902,
}

// Payload for SUPPORT_FUNCTION_INFO
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum SupportFunctionInfoType {
    IMAGE_SUPPORT_INFO = 0,
    BATTERY_INFO = 1,
    CAMERA_FUNCTION_INFO = 3,
    CAMERA_HISTORY_INFO = 4,
    PRINTER_FUNCTION_INFO = 5,
    PRINT_HISTORY_INFO = 6,
}

// Payload for DEVICE_INFO
#[allow(non_camel_case_types)]
pub enum DeviceInfoType {
    MANUFACTURER_NAME = 0,
    MODEL_NUMBER = 1,
    SERIAL_NUMBER = 2,
    HW_REVISION = 3,
    FW_REVISION = 4,
    SW_REVISION = 5,
    SYSTEM_ID = 6,
    REGULATORY_DATA = 7,
    PNP_ID = 8,
}

#[allow(non_camel_case_types)]
#[derive(Debug, FromPrimitive)]
pub enum CameraErrorType {
    NO_ERROR = -1,
    BATTERY_NG_ERROR = 0,
    NO_BATTERY_ERROR = 1,
    BATTERY_TEMP_ERROR = 2,
    BATTERY_CHARGE_FAULT_ERROR = 3,
    MEDIA_CAPACITY_FULL = 7,
    FRAME_NO_ERROR = 8,
    SW_ABNORMALITY_ERROR = 29,
    HW_ABNORMALITY_ERROR = 30,
    MECHA_ABNORMALITY_ERROR = 31,
    RESERVED_ERROR = -2,
}

#[allow(non_camel_case_types)]
pub enum ActiveMedia {
    SD = 0,
    BUILT_IN_MEDIA = 1,
    UNDEFINED = 255,
}

#[derive(Debug)]
pub struct ImageSupportInfo {
    pub width: u16,
    pub height: u16,
    pub pic_type: u8,
    pub pic_option: u8,
    pub size: u32,
}

impl ImageSupportInfo {
    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        assert_eq!(bytes[0], SupportFunctionInfoType::IMAGE_SUPPORT_INFO as u8);
        ImageSupportInfo {
            width: u16::from_be_bytes([bytes[1], bytes[2]]),
            height: u16::from_be_bytes([bytes[3], bytes[4]]),
            pic_type: bytes[5],
            pic_option: bytes[6],
            size: u32::from_be_bytes([bytes[7], bytes[8], bytes[9], bytes[10]]),
        }
    }
}

#[derive(Debug)]
pub struct BatteryInfo {
    pub battery_level: u8,
    pub battery_capacity: u8,
    pub charger_type: u8,
    pub charger_state: u8
}

impl BatteryInfo {
    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        assert_eq!(bytes[0], SupportFunctionInfoType::BATTERY_INFO as u8);
        BatteryInfo {
            battery_level: bytes[1],
            battery_capacity: bytes[2],
            charger_type: bytes[3],
            charger_state: bytes[4]
        }
    }
}

#[derive(Debug)]
pub struct CameraFunctionInfo {
    pub battery_level: u8,
    pub is_charging: bool,
    pub battery_capacity: u8,
    pub auto_image_transfer_count: u8,
    pub charger_state: u8,
    pub camera_error_type: CameraErrorType,
    pub camera_status: u8,
}

impl CameraFunctionInfo {
    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        assert_eq!(bytes[0], SupportFunctionInfoType::CAMERA_FUNCTION_INFO as u8);
        CameraFunctionInfo {
            battery_level: bytes[1] & 15,
            is_charging: (bytes[1] << 4 & 1) != 0,
            battery_capacity: bytes[2],
            auto_image_transfer_count: bytes[3],
            charger_state: bytes[4],
            camera_error_type: FromPrimitive::from_u16(u16::from_be_bytes([bytes[5], bytes[6]])).unwrap(),
            camera_status: bytes[7],
        }
    }
}

#[derive(Debug)]
pub struct CameraHistoryInfo {
    pub total_shoot_num: u32,
}

impl CameraHistoryInfo {
    pub fn from_bytes(bytes: &Vec<u8>) -> Self {
        assert_eq!(bytes[0], SupportFunctionInfoType::CAMERA_HISTORY_INFO as u8);
        CameraHistoryInfo {
            total_shoot_num: u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]),
        }
    }
}
