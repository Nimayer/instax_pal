// Instax protocol direction: to or from device
pub const DIRECTION_TO: u16 = 0x4162;   // "Ab"
pub const DIRECTION_FROM: u16 = 0x6142; // "aB"

// SID: Instax protocol opcodes
// u16 with modeCode, typeCode as big endian
pub const UNKNOWN: u16 = 0xffff;
pub const SUPPORT_FUNCTION_AND_VERSION_INFO: u16 = 0x0000;
pub const DEVICE_INFO_SERVICE: u16 = 0x0001;
pub const SUPPORT_FUNCTION_INFO: u16 = 0x0002;
pub const IDENTIFY_INFORMATION: u16 = 0x0010;
pub const SHUT_DOWN: u16 = 0x0100;
pub const RESET: u16 = 0x0101;
pub const AUTO_SLEEP_SETTINGS: u16 = 0x0102;
pub const BLE_CONNECT: u16 = 0x0103;
pub const PRINT_IMAGE_DOWNLOAD_START: u16 = 0x1000;
pub const PRINT_IMAGE_DOWNLOAD_DATA: u16 = 0x1001;
pub const PRINT_IMAGE_DOWNLOAD_END: u16 = 0x1002;
pub const PRINT_IMAGE_DOWNLOAD_CANCEL: u16 = 0x1003;
pub const PRINT_IMAGE: u16 = 0x1080;
pub const REJECT_FILM_COVER: u16 = 0x1081;
pub const FW_DOWNLOAD_START: u16 = 0x2000;
pub const FW_DOWNLOAD_DATA: u16 = 0x2001;
pub const FW_DOWNLOAD_END: u16 = 0x2002;
pub const FW_UPGRADE_EXIT: u16 = 0x2003;
pub const FW_PROGRAM_INFO: u16 = 0x2010;
pub const FW_DATA_BACKUP: u16 = 0x2080;
pub const FW_UPDATE_REQUEST: u16 = 0x2081;
pub const XYZ_AXIS_INFO: u16 = 0x3000;
pub const LED_PATTERN_SETTINGS: u16 = 0x3001;
pub const AXIS_ACTION_SETTINGS: u16 = 0x3002;
pub const LED_PATTERN_SETTINGS_DOUBLE: u16 = 0x3003;
pub const POWER_ONOFF_LED_SETTING: u16 = 0x3004;
pub const AR_LED_VIBRARTION_SETTING: u16 = 0x3006;
pub const FUNCTION_BUTTON_SETTING: u16 = 0x3008;
pub const ADDITIONAL_PRINTER_INFO: u16 = 0x3010;
pub const PRINTER_HEAD_LIGHT_CORRECT_INFO: u16 = 0x3080;
pub const PRINTER_HEAD_LIGHT_CORRECT_SETTINGS: u16 = 0x3081;
pub const CAMERA_SETTINGS: u16 = 0x8000;
pub const CAMERA_SETTINGS_GET: u16 = 0x8001;
pub const ADDITIONAL_CAMERA_INFO: u16 = 0x8010;
pub const PARAMETER_RW: u16 = 0x8011;
pub const TIME_SETTING: u16 = 0x8012;
pub const URL_UPLOAD_INFO: u16 = 0x8100;
pub const URL_PICTURE_UPLOAD_START: u16 = 0x8101;
pub const URL_PICTURE_UPLOAD: u16 = 0x8102;
pub const URL_PICTURE_UPLOAD_END: u16 = 0x8103;
pub const URL_AUDIO_UPLOAD_START: u16 = 0x8104;
pub const URL_AUDIO_UPLOAD: u16 = 0x8105;
pub const URL_AUDIO_UPLOAD_END: u16 = 0x8106;
pub const URL_UPLOAD_ADDRESS: u16 = 0x8107;
pub const URL_UPLOAD_DATA_COMPLETE: u16 = 0x8108;
pub const LIVE_VIEW_START: u16 = 0x8200;
pub const LIVE_VIEW_RECEIVE: u16 = 0x8201;
pub const LIVE_VIEW_STOP: u16 = 0x8202;
pub const LIVE_VIEW_TAKE_PICTURE: u16 = 0x8210;
pub const POST_VIEW_UPLOAD_START: u16 = 0x8220;
pub const POST_VIEW_UPLOAD: u16 = 0x8221;
pub const POST_VIEW_UPLOAD_END: u16 = 0x8222;
pub const POST_VIEW_PRINT: u16 = 0x8230;
pub const FRAME_PICTURE_DOWNLOAD_START: u16 = 0x8300;
pub const FRAME_PICTURE_DOWNLOAD: u16 = 0x8301;
pub const FRAME_PICTURE_DOWNLOAD_END: u16 = 0x8302;
pub const FRAME_PICTURE_NAME_SETTING: u16 = 0x8303;
pub const FRAME_PICTURE_NAME_GET: u16 = 0x8304;
pub const CAMERA_LOG_SUBTOTAL_START: u16 = 0x8400;
pub const CAMERA_LOG_SUBTOTAL_DATA: u16 = 0x8401;
pub const CAMERA_LOG_SUBTOTAL_CLEAR: u16 = 0x8402;
pub const CAMERA_LOG_DATE_START: u16 = 0x8403;
pub const CAMERA_LOG_DATE_DATA: u16 = 0x8404;
pub const CAMERA_LOG_DATE_CLEAR: u16 = 0x8405;
pub const CAMERA_LOG_FILTER_START: u16 = 0x8406;
pub const CAMERA_LOG_FILTER_DATA: u16 = 0x8407;
pub const CAMERA_LOG_FILTER_CLEAR: u16 = 0x8408;
pub const CAMERA_LOG_RECORD_DATE_START: u16 = 0x8409;
pub const CAMERA_LOG_RECORD_DATE_DATA: u16 = 0x840a;
pub const CAMERA_LOG_RECORD_DATE_CLEAR: u16 = 0x840b;
pub const CHECK_CAMERA_STATUS: u16 = 0x8500;
pub const EXECUTE_CAMERA_COMMAND: u16 = 0x8501;
pub const SOUND_PLAY_STATUS: u16 = 0x8600;
pub const SOUND_DOWNLOAD_START: u16 = 0x8601;
pub const SOUND_DOWNLOAD: u16 = 0x8602;
pub const SOUND_DOWNLOAD_END: u16 = 0x8603;
pub const SOUND_DOWNLOAD_CANCEL: u16 = 0x8604;
pub const SOUND_PLAY_START: u16 = 0x8605;
pub const SOUND_PLAY_STOP: u16 = 0x8606;
pub const IMAGE_MANUAL_UPLOAD_INFO: u16 = 0x8700;
pub const IMAGE_MANUAL_UPLOAD_START: u16 = 0x8701;
pub const IMAGE_MANUAL_UPLOAD_DATA: u16 = 0x8702;
pub const IMAGE_MANUAL_UPLOAD_END: u16 = 0x8703;
pub const IMAGE_MANUAL_UPLOAD_CANCEL: u16 = 0x8704;
pub const IMAGE_AUTO_UPLOAD_INFO: u16 = 0x8800;
pub const IMAGE_AUTO_UPLOAD_START: u16 = 0x8801;
pub const IMAGE_AUTO_UPLOAD_DATA: u16 = 0x8802;
pub const IMAGE_AUTO_UPLOAD_END: u16 = 0x8803;
pub const IMAGE_AUTO_UPLOAD_CANCEL: u16 = 0x8804;
pub const IMAGE_AUTO_UPLOAD_COMPLETE: u16 = 0x8805;
pub const INTERVAL_RECORD_STATUS: u16 = 0x8900;
pub const INTERVAL_RECORD_START: u16 = 0x8901;
pub const INTERVAL_RECORD_STOP: u16 = 0x8902;

// Payload for SUPPORT_FUNCTION_INFO
pub enum SupportFunctionInfoType { 
    IMAGE_SUPPORT_INFO = 0,
    BATTERY_INFO = 1,
    PRINTER_FUNCTION_INFO = 2,
    PRINT_HISTORY_INFO = 3,
    CAMERA_FUNCTION_INFO = 4,
    CAMERA_HISTORY_INFO = 5,
}

pub enum ActiveMedia {
    SD = 0,
    BUILT_IN_MEDIA = 1,
    UNDEFINED = 255,
}