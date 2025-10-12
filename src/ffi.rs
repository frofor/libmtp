//! This module provides external FFI bindings to libmtp library.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused)]

use libc::time_t;
use libc::timeval;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_uchar;
use std::ffi::c_uint;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::mem::ManuallyDrop;

pub type LIBMTP_event_t = LIBMTP_event_enum;
pub type LIBMTP_device_entry_t = LIBMTP_device_entry_struct;
pub type LIBMTP_raw_device_t = LIBMTP_raw_device_struct;
pub type LIBMTP_error_t = LIBMTP_error_struct;
pub type LIBMTP_allowed_values_t = LIBMTP_allowed_values_struct;
pub type LIBMTP_device_extension_t = LIBMTP_device_extension_struct;
pub type LIBMTP_mtpdevice_t = LIBMTP_mtpdevice_struct;
pub type LIBMTP_file_t = LIBMTP_file_struct;
pub type LIBMTP_track_t = LIBMTP_track_struct;
pub type LIBMTP_playlist_t = LIBMTP_playlist_struct;
pub type LIBMTP_album_t = LIBMTP_album_struct;
pub type LIBMTP_folder_t = LIBMTP_folder_struct;
pub type LIBMTP_filesampledata_t = LIBMTP_filesampledata_struct;
pub type LIBMTP_devicestorage_t = LIBMTP_devicestorage_struct;
pub type LIBMTP_progressfunc_t = Option<unsafe extern "C" fn(u64, u64, *const c_void)>;
pub type MTPDataGetFunc =
	Option<unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *mut c_uchar, *mut u32) -> u16>;
pub type MTPDataPutFunc =
	Option<unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *mut c_uchar, *mut u32) -> u16>;
pub type LIBMTP_event_cb_fn = Option<unsafe extern "C" fn(c_int, LIBMTP_event_t, u32, *mut c_void)>;
pub type PTPErrorFunc = fn(*mut c_void, *const c_char, *mut c_void);
pub type PTPDebugFunc = fn(*mut c_void, *const c_char, *mut c_void);
pub type PTPDataGetFunc =
	fn(*mut PTPParams, *mut c_void, c_ulong, *mut c_uchar, *mut c_ulong) -> u16;
pub type PTPDataPutFunc = fn(*mut PTPParams, *mut c_void, c_ulong, *mut c_uchar) -> u16;
pub type PTPIOSendReq = fn(*mut PTPParams, *mut PTPContainer, c_int) -> u16;
pub type PTPIOSendData = fn(*mut PTPParams, *mut PTPContainer, u64, *mut PTPDataHandler) -> u16;
pub type PTPIOGetResp = fn(*mut PTPParams, *mut PTPContainer) -> u16;
pub type PTPIOGetData = fn(*mut PTPParams, *mut PTPContainer, *mut PTPDataHandler) -> u16;
pub type PTPIOCancelReq = fn(*mut PTPParams, u32) -> u16;
pub type PTPIODevStatReq = fn(*mut PTPParams) -> u16;

pub const LIBMTP_STORAGE_SORTBY_NOTSORTED: c_int = 0;
pub const LIBMTP_STORAGE_SORTBY_FREESPACE: c_int = 1;
pub const LIBMTP_STORAGE_SORTBY_MAXSPACE: c_int = 2;
pub const LIBMTP_FILES_AND_FOLDERS_ROOT: u32 = 4_294_967_295;
pub const PTP_ST_Undefined: u16 = 0;
pub const PTP_ST_FixedROM: u16 = 1;
pub const PTP_ST_RemovableROM: u16 = 2;
pub const PTP_ST_FixedRAM: u16 = 3;
pub const PTP_ST_RemovableRAM: u16 = 4;
pub const PTP_FST_Undefined: u16 = 0;
pub const PTP_FST_GenericFlat: u16 = 1;
pub const PTP_FST_GenericHierarchical: u16 = 2;
pub const PTP_FST_DCF: u16 = 3;
pub const PTP_AC_ReadWrite: u16 = 0;
pub const PTP_AC_ReadOnly: u16 = 1;
pub const PTP_AC_ReadOnly_with_Object_Deletion: u16 = 2;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LIBMTP_event_enum {
	None,
	StoreAdded,
	StoreRemoved,
	ObjectAdded,
	ObjectRemoved,
	DevicePropertyChanged,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LIBMTP_filetype_t {
	Folder,
	Wav,
	Mp3,
	Wma,
	Ogg,
	Audible,
	Mp4,
	UndefAudio,
	Wmv,
	Avi,
	Mpeg,
	Asf,
	Qt,
	UndefVideo,
	Jpeg,
	Jfif,
	Tiff,
	Bmp,
	Gif,
	Pict,
	Png,
	VCalendar1,
	VCalendar2,
	VCard2,
	VCard3,
	WindowsImageFormat,
	WinExec,
	Text,
	Html,
	Firmware,
	Aac,
	MediaCard,
	Flac,
	Mp2,
	M4a,
	Doc,
	Xml,
	Xls,
	Ppt,
	Mht,
	Jp2,
	Jpx,
	Album,
	Playlist,
	Unknown,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LIBMTP_property_t {
	StorageId,
	ObjectFormat,
	ProtectionStatus,
	ObjectSize,
	AssociationType,
	AssociationDesc,
	ObjectFileName,
	DateCreated,
	DateModified,
	Keywords,
	ParentObject,
	AllowedFolderContents,
	Hidden,
	SystemObject,
	PersistantUniqueObjectIdentifier,
	SyncId,
	PropertyBag,
	Name,
	CreatedBy,
	Artist,
	DateAuthored,
	Description,
	UrlReference,
	LanguageLocale,
	CopyrightInformation,
	Source,
	OriginLocation,
	DateAdded,
	NonConsumable,
	CorruptOrUnplayable,
	ProducerSerialNumber,
	RepresentativeSampleFormat,
	RepresentativeSampleSize,
	RepresentativeSampleHeight,
	RepresentativeSampleWidth,
	RepresentativeSampleDuration,
	RepresentativeSampleData,
	Width,
	Height,
	Duration,
	Rating,
	Track,
	Genre,
	Credits,
	Lyrics,
	SubscriptionContentId,
	ProducedBy,
	UseCount,
	SkipCount,
	LastAccessed,
	ParentalRating,
	MetaGenre,
	Composer,
	EffectiveRating,
	Subtitle,
	OriginalReleaseDate,
	AlbumName,
	AlbumArtist,
	Mood,
	DrmStatus,
	SubDescription,
	IsCropped,
	IsColorCorrected,
	ImageBitDepth,
	Fnumber,
	ExposureTime,
	ExposureIndex,
	DisplayName,
	BodyText,
	Subject,
	Priority,
	GivenName,
	MiddleNames,
	FamilyName,
	Prefix,
	Suffix,
	PhoneticGivenName,
	PhoneticFamilyName,
	EmailPrimary,
	EmailPersonal1,
	EmailPersonal2,
	EmailBusiness1,
	EmailBusiness2,
	EmailOthers,
	PhoneNumberPrimary,
	PhoneNumberPersonal,
	PhoneNumberPersonal2,
	PhoneNumberBusiness,
	PhoneNumberBusiness2,
	PhoneNumberMobile,
	PhoneNumberMobile2,
	FaxNumberPrimary,
	FaxNumberPersonal,
	FaxNumberBusiness,
	PagerNumber,
	PhoneNumberOthers,
	PrimaryWebAddress,
	PersonalWebAddress,
	BusinessWebAddress,
	InstantMessengerAddress,
	InstantMessengerAddress2,
	InstantMessengerAddress3,
	PostalAddressPersonalFull,
	PostalAddressPersonalFullLine1,
	PostalAddressPersonalFullLine2,
	PostalAddressPersonalFullCity,
	PostalAddressPersonalFullRegion,
	PostalAddressPersonalFullPostalCode,
	PostalAddressPersonalFullCountry,
	PostalAddressBusinessFull,
	PostalAddressBusinessLine1,
	PostalAddressBusinessLine2,
	PostalAddressBusinessCity,
	PostalAddressBusinessRegion,
	PostalAddressBusinessPostalCode,
	PostalAddressBusinessCountry,
	PostalAddressOtherFull,
	PostalAddressOtherLine1,
	PostalAddressOtherLine2,
	PostalAddressOtherCity,
	PostalAddressOtherRegion,
	PostalAddressOtherPostalCode,
	PostalAddressOtherCountry,
	OrganizationName,
	PhoneticOrganizationName,
	Role,
	Birthdate,
	MessageTo,
	MessageCc,
	MessageBcc,
	MessageRead,
	MessageReceivedTime,
	MessageSender,
	ActivityBeginTime,
	ActivityEndTime,
	ActivityLocation,
	ActivityRequiredAttendees,
	ActivityOptionalAttendees,
	ActivityResources,
	ActivityAccepted,
	Owner,
	Editor,
	Webmaster,
	UrlSource,
	UrlDestination,
	TimeBookmark,
	ObjectBookmark,
	ByteBookmark,
	LastBuildDate,
	TimetoLive,
	MediaGuid,
	TotalBitRate,
	BitRateType,
	SampleRate,
	NumberOfChannels,
	AudioBitDepth,
	ScanDepth,
	AudioWaveCodec,
	AudioBitRate,
	VideoFourCcCodec,
	VideoBitRate,
	FramesPerThousandSeconds,
	KeyFrameDistance,
	BufferSize,
	EncodingQuality,
	EncodingProfile,
	BuyFlag,
	Unknown,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LIBMTP_datatype_t {
	Int8,
	Uint8,
	Int16,
	Uint16,
	Int32,
	Uint32,
	Int64,
	Uint64,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LIBMTP_devicecap_t {
	GetPartialObject,
	SendPartialObject,
	EditObjects,
	MoveObject,
	CopyObject,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LIBMTP_error_number_t {
	None,
	General,
	PtpLayer,
	UsbLayer,
	MemoryAllocation,
	NoDeviceAttached,
	StorageFull,
	Connecting,
	Cancelled,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_device_entry_struct {
	pub vendor: *mut c_char,
	pub vendor_id: u16,
	pub product: *mut c_char,
	pub product_id: u16,
	pub device_flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_raw_device_struct {
	pub device_entry: LIBMTP_device_entry_t,
	pub bus_location: u32,
	pub devnum: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_error_struct {
	pub errornumber: LIBMTP_error_number_t,
	pub error_text: *mut c_char,
	pub next: *mut LIBMTP_error_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_allowed_values_struct {
	pub u8max: u8,
	pub u8min: u8,
	pub u8step: u8,
	pub u8vals: *mut u8,
	pub i8max: i8,
	pub i8min: i8,
	pub i8step: i8,
	pub i8vals: *mut i8,
	pub u16max: u16,
	pub u16min: u16,
	pub u16step: u16,
	pub u16vals: *mut u16,
	pub i16max: i16,
	pub i16min: i16,
	pub i16step: i16,
	pub i16vals: *mut i16,
	pub u32max: u32,
	pub u32min: u32,
	pub u32step: u32,
	pub u32vals: *mut u32,
	pub i32max: i32,
	pub i32min: i32,
	pub i32step: i32,
	pub i32vals: *mut i32,
	pub u64max: u64,
	pub u64min: u64,
	pub u64step: u64,
	pub u64vals: *mut u64,
	pub i64max: i64,
	pub i64min: i64,
	pub i64step: i64,
	pub i64vals: *mut i64,
	pub num_entries: u16,
	pub datatype: LIBMTP_datatype_t,
	pub is_range: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_device_extension_struct {
	pub name: *mut c_char,
	pub major: c_int,
	pub minor: c_int,
	pub next: *mut LIBMTP_device_extension_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_mtpdevice_struct {
	pub object_bitsize: u8,
	pub params: *mut c_void,
	pub usbinfo: *mut c_void,
	pub storage: *mut LIBMTP_devicestorage_t,
	pub errorstack: *mut LIBMTP_error_t,
	pub maximum_battery_level: u8,
	pub default_music_folder: u32,
	pub default_playlist_folder: u32,
	pub default_picture_folder: u32,
	pub default_video_folder: u32,
	pub default_organizer_folder: u32,
	pub default_zencast_folder: u32,
	pub default_album_folder: u32,
	pub default_text_folder: u32,
	pub cd: *mut c_void,
	pub extensions: *mut LIBMTP_device_extension_t,
	pub cached: c_int,
	pub next: *mut LIBMTP_mtpdevice_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_file_struct {
	pub item_id: u32,
	pub parent_id: u32,
	pub storage_id: u32,
	pub filename: *mut c_char,
	pub filesize: u64,
	pub modificationdate: time_t,
	pub filetype: LIBMTP_filetype_t,
	pub next: *mut LIBMTP_file_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_track_struct {
	pub item_id: u32,
	pub parent_id: u32,
	pub storage_id: u32,
	pub title: *mut c_char,
	pub artist: *mut c_char,
	pub composer: *mut c_char,
	pub genre: *mut c_char,
	pub album: *mut c_char,
	pub date: *mut c_char,
	pub filename: *mut c_char,
	pub tracknumber: u16,
	pub duration: u32,
	pub samplerate: u32,
	pub nochannels: u16,
	pub wavecodec: u32,
	pub bitrate: u32,
	pub bitratetype: u16,
	pub rating: u16,
	pub usecount: u32,
	pub filesize: u64,
	pub modificationdate: time_t,
	pub filetype: LIBMTP_filetype_t,
	pub next: *mut LIBMTP_track_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_playlist_struct {
	pub playlist_id: u32,
	pub parent_id: u32,
	pub storage_id: u32,
	pub name: *mut c_char,
	pub tracks: *mut u32,
	pub no_tracks: *mut u32,
	pub next: *mut LIBMTP_playlist_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_album_struct {
	pub album_id: u32,
	pub parent_id: u32,
	pub storage_id: u32,
	pub name: *mut c_char,
	pub artist: *mut c_char,
	pub composer: *mut c_char,
	pub genre: *mut c_char,
	pub tracks: *mut u32,
	pub no_tracks: *mut u32,
	pub next: *mut LIBMTP_album_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_folder_struct {
	pub folder_id: u32,
	pub parent_id: u32,
	pub storage_id: u32,
	pub name: *mut c_char,
	pub sibling: *mut LIBMTP_folder_t,
	pub child: *mut LIBMTP_folder_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_filesampledata_struct {
	pub width: u32,
	pub height: u32,
	pub duration: u32,
	pub filetype: LIBMTP_filetype_t,
	pub size: u64,
	pub data: *mut c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct LIBMTP_devicestorage_struct {
	pub id: u32,
	pub StorageType: u16,
	pub FilesystemType: u16,
	pub AccessCapability: u16,
	pub MaxCapacity: u64,
	pub FreeSpaceInBytes: u64,
	pub FreeSpaceInObjects: u64,
	pub StorageDescription: *mut c_char,
	pub VolumeIdentifier: *mut c_char,
	pub next: *mut LIBMTP_devicestorage_t,
	pub prev: *mut LIBMTP_devicestorage_t,
}

#[repr(C)]
#[derive(Clone)]
pub struct PTPParams {
	pub device_flags: u32,
	pub byteorder: u8,
	pub maxpacketsize: u16,
	pub sendreq_func: PTPIOSendReq,
	pub senddata_func: PTPIOSendData,
	pub getresp_func: PTPIOGetResp,
	pub getdata_func: PTPIOGetData,
	pub event_check: PTPIOGetResp,
	pub event_check_queue: PTPIOGetResp,
	pub event_wait: PTPIOGetResp,
	pub cancelreq_func: PTPIOCancelReq,
	pub devstatreq_func: PTPIODevStatReq,
	pub error_func: PTPErrorFunc,
	pub debug_func: PTPDebugFunc,
	pub data: *mut c_void,
	pub transaction_id: u32,
	pub session_id: u32,
	pub opencapture_transid: u32,
	pub split_header_data: c_int,
	pub ocs64: c_int,
	pub nrofobjectformats: c_int,
	pub object_formats: *mut MTPObjectFormat,
	pub objects: *mut PTPObject,
	pub nrofobjects: c_uint,
	pub deviceinfo: PTPDeviceInfo,
	pub events: *mut PTPContainer,
	pub nrofevents: c_uint,
	pub capcnt: c_uint,
	pub inlineview: c_int,
	pub cachetime: c_int,
	pub storageids: PTPStorageIDs,
	pub storagechanged: c_int,
	pub deviceproperties: *mut PTPDeviceProperty,
	pub nrofdeviceproperties: c_uint,
	pub canon_proprs: *mut PTPCanon_Property,
	pub nrofcanon_props: c_uint,
	pub canon_viewfinder_on: c_int,
	pub canon_event_mode: c_int,
	pub backlogentries: *mut PTPCanon_changes_entry,
	pub nrofbacklogentries: c_uint,
	pub eos_captureenabled: c_int,
	pub eos_camerastatus: c_int,
	pub controlmode: c_int,
	pub event90c7works: c_int,
	pub deletesdramfails: c_int,
	pub starttime: timeval,
	pub wifi_profiles_version: u8,
	pub wifi_profiles_number: u8,
	pub wifi_profiles: *mut PTPNIKONWifiProfile,
	pub cmdfd: c_int,
	pub evtfd: c_int,
	pub jpgfd: c_int,
	pub cameraguid: [u8; 16],
	pub eventpipeid: u32,
	pub cameraname: *mut c_char,
	pub outer_deviceinfo: PTPDeviceInfo,
	pub olympus_cmd: *mut c_char,
	pub olympus_reply: *mut c_char,
	pub outer_params: Box<PTPParams>,
	pub response_packet: *mut u8,
	pub response_packet_size: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct PTPDeviceInfo {
	pub StandardVersion: u16,
	pub VendorExtensionID: u32,
	pub VendorExtensionDesc: *mut c_char,
	pub FunctionalMode: u16,
	pub OperationsSupported_len: u32,
	pub OperationsSupported: *mut u16,
	pub EventsSupported_len: u32,
	pub EventsSupported: *mut u16,
	pub DevicePropertiesSupported_len: u32,
	pub DevicePropertiesSupported: *mut u16,
	pub CaptureFormats_len: u32,
	pub CaptureFormats: *mut u16,
	pub ImageFormats_len: u32,
	pub ImageFormats: *mut u16,
	pub Manufacturer: *mut c_char,
	pub Model: *mut c_char,
	pub DeviceVersion: *mut c_char,
	pub SerialNumber: *mut c_char,
}

#[repr(C)]
pub struct PTPDeviceProperty {
	pub timestamp: time_t,
	pub desc: PTPDevicePropDesc,
	pub value: PTPPropertyValue,
}

#[repr(C)]
pub struct PTPDevicePropDesc {
	pub DevicePropertyCode: u16,
	pub DataType: u16,
	pub GetSet: u8,
	pub FactoryDefaultValue: PTPPropertyValue,
	pub CurrentValue: PTPPropertyValue,
	pub FormFlag: u8,
	pub FORM: PTPDevicePropDescForm,
}

#[repr(C)]
pub union PTPDevicePropDescForm {
	pub Enum: PTPPropDescEnumForm,
	pub Range: ManuallyDrop<PTPPropDescRangeForm>,
}

#[repr(C)]
pub struct PTPCanon_Property {
	pub size: u32,
	pub proptype: u32,
	pub data: *mut c_uchar,
	pub dpd: PTPDevicePropDesc,
}

#[repr(C)]
pub struct PTPCanon_changes_entry {
	pub r#type: PTPCanon_changes_types,
	pub u: PTPCanon_changes_entryUnion,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PTPCanon_changes_types {
	PTP_CANON_EOS_CHANGES_TYPE_UNKNOWN,
	PTP_CANON_EOS_CHANGES_TYPE_OBJECTINFO,
	PTP_CANON_EOS_CHANGES_TYPE_OBJECTTRANSFER,
	PTP_CANON_EOS_CHANGES_TYPE_PROPERTY,
	PTP_CANON_EOS_CHANGES_TYPE_CAMERASTATUS,
	PTP_CANON_EOS_CHANGES_TYPE_FOCUSINFO,
	PTP_CANON_EOS_CHANGES_TYPE_FOCUSMASK,
	PTP_CANON_EOS_CHANGES_TYPE_OBJECTREMOVED,
	PTP_CANON_EOS_CHANGES_TYPE_OBJECTINFO_CHANGE,
	PTP_CANON_EOS_CHANGES_TYPE_OBJECTCONTENT_CHANGE,
}

#[repr(C)]
pub union PTPCanon_changes_entryUnion {
	pub object: PTPCanon_New_Object,
	pub info: *mut c_char,
	pub propid: u16,
	pub status: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct PTPCanon_New_Object {
	pub oid: u32,
	pub oi: PTPObjectInfo,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct PTPObjectInfo {
	pub StorageID: u32,
	pub ObjectFormat: u16,
	pub ProtectionStatus: u16,
	pub ObjectCompressedSize: u64,
	pub ThumbFormat: u16,
	pub ThumbCompressedSize: u32,
	pub ThumbPixWidth: u32,
	pub ThumbPixHeight: u32,
	pub ImagePixWidth: u32,
	pub ImagePixHeight: u32,
	pub ImageBitDepth: u32,
	pub ParentObject: u32,
	pub AssociationType: u16,
	pub AssociationDesc: u32,
	pub SequenceNumber: u32,
	pub Filename: *mut c_char,
	pub CaptureDate: time_t,
	pub ModificationDate: time_t,
	pub Keywords: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PTPNIKONWifiProfile {
	pub profile_name: [c_char; 17],
	pub device_type: u8,
	pub icon_type: u8,
	pub essid: [c_char; 33],
	pub id: u8,
	pub valid: u8,
	pub display_order: u8,
	pub creation_date: [c_char; 16],
	pub lastusage_date: [c_char; 16],
	pub ip_address: u32,
	pub subnet_mask: u8,
	pub gateway_address: u32,
	pub address_mode: u8,
	pub access_mode: u8,
	pub wifi_channel: u8,
	pub authentification: u8,
	pub encryption: u8,
	pub key: [u8; 64],
	pub key_nr: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct PTPContainer {
	pub Code: u16,
	pub SessionID: u32,
	pub Transaction_ID: u32,
	pub Param1: u32,
	pub Param2: u32,
	pub Param3: u32,
	pub Param4: u32,
	pub Param5: u32,
	pub Nparam: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct MTPObjectFormat {
	pub ofc: u16,
	pub nrofpds: c_uint,
	pub pds: *mut MTPPropertyDesc,
}

#[repr(C)]
pub struct MTPPropertyDesc {
	pub opc: u16,
	pub opd: PTPObjectPropDesc,
}

#[repr(C)]
pub struct PTPObjectPropDesc {
	pub ObjectPropertyCode: u16,
	pub DataType: u16,
	pub GetSet: u8,
	pub FactoryDefaultValue: PTPPropertyValue,
	pub GroupCode: u32,
	pub FormFlag: u8,
	pub FORM: PTPObjectPropDescForm,
}

#[repr(C)]
pub union PTPObjectPropDescForm {
	pub Enum: PTPPropDescEnumForm,
	pub Range: ManuallyDrop<PTPPropDescRangeForm>,
	pub DateTime: PTPPropDescStringForm,
	pub FixedLengthArray: PTPPropDescArrayLengthForm,
	pub RegularExpression: PTPPropDescStringForm,
	pub ByteArray: PTPPropDescArrayLengthForm,
	pub LongString: PTPPropDescStringForm,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct PTPPropDescEnumForm {
	pub NumberOfValues: u16,
	pub SupportedValues: *mut PTPPropertyValue,
}

#[repr(C)]
pub struct PTPPropDescRangeForm {
	pub MinimumValue: PTPPropertyValue,
	pub MaximumValue: PTPPropertyValue,
	pub StepSize: PTPPropertyValue,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct PTPPropDescStringForm {
	pub Strict: *mut c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct PTPPropDescArrayLengthForm {
	pub NumberOfValues: u16,
}

#[repr(C)]
pub union PTPPropertyValue {
	pub str: *mut c_char,
	pub u8: u8,
	pub i8: i8,
	pub u16: u16,
	pub i16: i16,
	pub u32: u32,
	pub i32: i32,
	pub u64: u64,
	pub i64: i64,
	pub a: ManuallyDrop<Box<PTPPropertyValueArray>>,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct PTPObject {
	pub oid: u32,
	pub flags: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct PTPStorageIDs {
	pub n: u32,
	pub Storage: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct PTPPropertyValueArray {
	pub count: u32,
	pub v: *mut PTPPropertyValue,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub struct PTPDataHandler {
	pub getfunc: PTPDataGetFunc,
	pub putfunc: PTPDataPutFunc,
	pub r#priv: *mut c_void,
}

#[link(name = "mtp")]
unsafe extern "C" {
	pub static mut LIBMTP_debug: c_int;
	pub fn LIBMTP_Set_Debug(level: c_int);
	pub fn LIBMTP_Init();
	pub fn LIBMTP_Get_Supported_Devices_List(
		devices: *mut *mut LIBMTP_device_entry_t,
		numdevs: *mut c_int,
	) -> i32;
	pub fn LIBMTP_Detect_Raw_Devices(
		devices: *mut *mut LIBMTP_raw_device_t,
		numdevs: *mut c_int,
	) -> LIBMTP_error_number_t;
	pub fn LIBMTP_Check_Specific_Device(busno: c_int, devno: c_int) -> c_int;
	pub fn LIBMTP_Open_Raw_Device(rawdevice: *mut LIBMTP_raw_device_t) -> *mut LIBMTP_mtpdevice_t;
	pub fn LIBMTP_Open_Raw_Device_Uncached(
		rawdevice: *mut LIBMTP_raw_device_t,
	) -> *mut LIBMTP_mtpdevice_t;
	pub fn LIBMTP_Get_Device(device_nr: c_int) -> *mut LIBMTP_mtpdevice_t;
	pub fn LIBMTP_Get_First_Device() -> *mut LIBMTP_mtpdevice_t;
	pub fn LIBMTP_Get_Device_By_SerialNumber(
		serial_number: *const c_char,
	) -> *mut LIBMTP_mtpdevice_t;
	pub fn LIBMTP_Get_Device_By_ID(device_id: *const c_char) -> *mut LIBMTP_mtpdevice_t;
	pub fn LIBMTP_Get_Connected_Devices(
		device_list: *mut *mut LIBMTP_mtpdevice_t,
	) -> LIBMTP_error_number_t;
	pub fn LIBMTP_Number_Devices_In_List(device_list: *mut LIBMTP_mtpdevice_t) -> u32;
	pub fn LIBMTP_Release_Device_List(device: *mut LIBMTP_mtpdevice_t);
	pub fn LIBMTP_Release_Device(device: *mut LIBMTP_mtpdevice_t);
	pub fn LIBMTP_Dump_Device_Info(device: *mut LIBMTP_mtpdevice_t);
	pub fn LIBMTP_Reset_Device(device: *mut LIBMTP_mtpdevice_t) -> c_int;
	pub fn LIBMTP_Get_Manufacturername(device: *mut LIBMTP_mtpdevice_t) -> *mut c_char;
	pub fn LIBMTP_Get_Modelname(device: *mut LIBMTP_mtpdevice_t) -> *mut c_char;
	pub fn LIBMTP_Get_Serialnumber(device: *mut LIBMTP_mtpdevice_t) -> *mut c_char;
	pub fn LIBMTP_Get_Deviceversion(device: *mut LIBMTP_mtpdevice_t) -> *mut c_char;
	pub fn LIBMTP_Get_Friendlyname(device: *mut LIBMTP_mtpdevice_t) -> *mut c_char;
	pub fn LIBMTP_Set_Friendlyname(
		device: *mut LIBMTP_mtpdevice_t,
		friendlyname: *const c_char,
	) -> c_int;
	pub fn LIBMTP_Get_Syncpartner(device: *mut LIBMTP_mtpdevice_t) -> *mut c_char;
	pub fn LIBMTP_Set_Syncpartner(
		device: *mut LIBMTP_mtpdevice_t,
		syncpartner: *const c_char,
	) -> c_int;
	pub fn LIBMTP_Get_Batterylevel(
		device: *mut LIBMTP_mtpdevice_t,
		maximum_level: *mut u8,
		current_level: *mut u8,
	) -> c_int;
	pub fn LIBMTP_Get_Secure_Time(
		device: *mut LIBMTP_mtpdevice_t,
		sectime: *mut *mut c_char,
	) -> c_int;
	pub fn LIBMTP_Get_Device_Certificate(
		device: *mut LIBMTP_mtpdevice_t,
		devcert: *mut *mut c_char,
	) -> c_int;
	pub fn LIBMTP_Get_Supported_Filetypes(
		device: *mut LIBMTP_mtpdevice_t,
		filetypes: *mut *mut u16,
		length: *mut u16,
	) -> c_int;
	pub fn LIBMTP_Check_Capability(
		device: *mut LIBMTP_mtpdevice_t,
		cap: LIBMTP_devicecap_t,
	) -> c_int;
	pub fn LIBMTP_Get_Errorstack(device: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_error_t;
	pub fn LIBMTP_Clear_Errorstack(device: *mut LIBMTP_mtpdevice_t);
	pub fn LIBMTP_Dump_Errorstack(device: *mut LIBMTP_mtpdevice_t);
	pub fn LIBMTP_FreeMemory(mem: *mut c_void);
	pub fn LIBMTP_Get_Storage(device: *mut LIBMTP_mtpdevice_t, sortby: c_int) -> c_int;
	pub fn LIBMTP_Format_Storage(
		device: *mut LIBMTP_mtpdevice_t,
		storage: *mut LIBMTP_devicestorage_t,
	) -> c_int;
	pub fn LIBMTP_Get_String_From_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
	) -> *mut c_char;
	pub fn LIBMTP_Get_u64_From_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value_default: u64,
	) -> u64;
	pub fn LIBMTP_Get_u32_From_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value_default: u32,
	) -> u32;
	pub fn LIBMTP_Get_u16_From_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value_default: u16,
	) -> u16;
	pub fn LIBMTP_Get_u8_From_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value_default: u8,
	) -> u8;
	pub fn LIBMTP_Set_Object_String(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		string: *const c_char,
	) -> c_int;
	pub fn LIBMTP_Set_Object_u32(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value: u32,
	) -> c_int;
	pub fn LIBMTP_Set_Object_u16(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value: u16,
	) -> c_int;
	pub fn LIBMTP_Set_Object_u8(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value: u8,
	) -> c_int;
	pub fn LIBMTP_Get_Property_Description(inproperty: LIBMTP_property_t) -> *const c_char;
	pub fn LIBMTP_Is_Property_Supported(
		device: *mut LIBMTP_mtpdevice_t,
		property: LIBMTP_property_t,
		filetype: LIBMTP_filetype_t,
	) -> c_int;
	pub fn LIBTP_Get_Allowed_Property_Values(
		device: *mut LIBMTP_mtpdevice_t,
		property: LIBMTP_property_t,
		filetype: LIBMTP_filetype_t,
		allowed_vals: *mut LIBMTP_allowed_values_t,
	) -> c_int;
	pub fn LIBMTP_destroy_allowed_values_t(allowed_vals: *mut LIBMTP_allowed_values_t);
	pub fn LIBMTP_new_file_t() -> *mut LIBMTP_file_t;
	pub fn LIBMTP_destroy_file_t(file: *mut LIBMTP_file_t);
	pub fn LIBMTP_Get_Filetype_Description(intype: LIBMTP_filetype_t) -> *const c_char;
	pub fn LIBMTP_Get_Filelisting(device: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_file_t;
	pub fn LIBMTP_Get_Filelisting_With_Callback(
		device: *mut LIBMTP_mtpdevice_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> *mut LIBMTP_file_t;
	pub fn LIBMTP_Get_Files_And_Folders(
		device: *mut LIBMTP_mtpdevice_t,
		storage: u32,
		parent: u32,
	) -> *mut LIBMTP_file_t;
	pub fn LIBMTP_Get_Children(
		device: *mut LIBMTP_mtpdevice_t,
		storage: u32,
		parent: u32,
		out: *mut *mut u32,
	) -> c_int;
	pub fn LIBMTP_Get_Filemetadata(
		device: *mut LIBMTP_mtpdevice_t,
		fileid: u32,
	) -> *mut LIBMTP_file_t;
	pub fn LIBMTP_Get_File_To_File(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		path: *const c_char,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub fn LIBMTP_Get_File_To_File_Descriptor(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		fd: c_int,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub fn LIBMTP_Get_File_To_Handler(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		put_func: MTPDataPutFunc,
		r#priv: *mut c_void,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub fn LIBMTP_Send_File_From_File(
		device: *mut LIBMTP_mtpdevice_t,
		path: *const c_char,
		filedata: *mut LIBMTP_file_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub fn LIBMTP_Send_File_From_File_Descriptor(
		device: *mut LIBMTP_mtpdevice_t,
		fd: c_int,
		filedata: *mut LIBMTP_file_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub fn LIBMTP_Send_File_From_Handler(
		device: *mut LIBMTP_mtpdevice_t,
		get_func: MTPDataPutFunc,
		r#priv: *mut c_void,
		filedata: *mut LIBMTP_file_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub fn LIBMTP_Set_File_Name(
		device: *mut LIBMTP_mtpdevice_t,
		file: *mut LIBMTP_file_t,
		newname: *const c_char,
	) -> c_int;
	pub fn LIBMTP_new_filesampledata_t() -> *mut LIBMTP_filesampledata_t;
	pub fn LIBMTP_destroy_filesampledata_t(sample: *mut LIBMTP_filesampledata_t);
	pub fn LIBMTP_Get_Representative_Sample_Format(
		device: *mut LIBMTP_mtpdevice_t,
		filetype: LIBMTP_filetype_t,
		sample: *mut *mut LIBMTP_filesampledata_t,
	) -> c_int;
	pub fn LIBMTP_Get_Thumbnail(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		data: *mut *mut c_uchar,
		size: *mut c_uint,
	) -> c_int;
	pub fn LIBMTP_new_track_t() -> *mut LIBMTP_track_t;
	pub fn LIBMTP_destroy_track_t(track: *mut LIBMTP_track_t);
	pub fn LIBMTP_Get_Tracklisting(device: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_track_t;
	pub fn LIBMTP_Get_Tracklisting_With_Callback(
		device: *mut LIBMTP_mtpdevice_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> *mut LIBMTP_track_t;
	pub fn LIBMTP_Get_Tracklisting_With_Callback_For_Storage(
		device: *mut LIBMTP_mtpdevice_t,
		storage_id: u32,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> *mut LIBMTP_track_t;
	pub fn LIBMTP_Get_Trackmetadata(
		device: *mut LIBMTP_mtpdevice_t,
		trackid: u32,
	) -> *mut LIBMTP_track_t;
	pub fn LIBMTP_Get_Track_To_File(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		path: *const c_char,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub fn LIBMTP_Get_Track_To_File_Descriptor(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		fd: c_int,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub fn LIBMTP_Get_Track_To_Handler(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		put_func: MTPDataPutFunc,
		r#priv: *mut c_void,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub fn LIBMTP_Send_Track_From_File(
		device: *mut LIBMTP_mtpdevice_t,
		a: *const c_char,
		track: *const LIBMTP_track_t,
		progress: LIBMTP_progressfunc_t,
		a: *const c_void,
	) -> c_int;
	pub fn LIBMTP_Send_Track_From_File_Descriptor(
		device: *mut LIBMTP_mtpdevice_t,
		path: *const c_char,
		metadata: *const LIBMTP_track_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub fn LIBMTP_Send_Track_From_Handler(
		device: *mut LIBMTP_mtpdevice_t,
		get_func: MTPDataGetFunc,
		r#priv: *mut c_void,
		metadata: *const LIBMTP_track_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub fn LIBMTP_Update_Track_Metadata(
		device: *mut LIBMTP_mtpdevice_t,
		metadata: *const LIBMTP_track_t,
	) -> c_int;
	pub fn LIBMTP_Track_Exists(device: *mut LIBMTP_mtpdevice_t, id: u32) -> c_int;
	pub fn LIBMTP_Set_Track_Name(
		device: *mut LIBMTP_mtpdevice_t,
		track: *mut LIBMTP_track_t,
		newname: *const c_char,
	) -> c_int;
	pub fn LIBMTP_new_folder_t() -> *mut LIBMTP_folder_t;
	pub fn LIBMTP_destroy_folder_t(folder: *mut LIBMTP_folder_t);
	pub fn LIBMTP_Get_Folder_List(device: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_folder_t;
	pub fn LIBMTP_Get_Folder_List_For_Storage(
		device: *mut LIBMTP_mtpdevice_t,
		storage: u32,
	) -> *mut LIBMTP_folder_t;
	pub fn LIBMTP_Find_Folder(folderlist: *mut LIBMTP_folder_t, id: u32) -> *mut LIBMTP_folder_t;
	pub fn LIBMTP_Create_Folder(
		device: *mut LIBMTP_mtpdevice_t,
		name: *mut c_char,
		parent_id: u32,
		storage_id: u32,
	) -> u32;
	pub fn LIBMTP_Set_Folder_Name(
		device: *mut LIBMTP_mtpdevice_t,
		folder: *mut LIBMTP_folder_t,
		newname: *const c_char,
	) -> c_int;
	pub fn LIBMTP_new_playlist_t() -> *mut LIBMTP_playlist_t;
	pub fn LIBMTP_destroy_playlist_t(playlist: *mut LIBMTP_playlist_t);
	pub fn LIBMTP_Get_Playlist_List(device: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_playlist_t;
	pub fn LIBMTP_Get_Playlist(
		device: *mut LIBMTP_mtpdevice_t,
		plid: u32,
	) -> *mut LIBMTP_playlist_t;
	pub fn LIBMTP_Create_New_Playlist(
		device: *mut LIBMTP_mtpdevice_t,
		metadata: *const LIBMTP_playlist_t,
	) -> c_int;
	pub fn LIBMTP_Update_Playlist(
		device: *mut LIBMTP_mtpdevice_t,
		metadata: *const LIBMTP_playlist_t,
	) -> c_int;
	pub fn LIBMTP_Set_Playlist_Name(
		device: *mut LIBMTP_mtpdevice_t,
		playlist: *mut LIBMTP_playlist_t,
		newname: *const c_char,
	) -> c_int;
	pub fn LIBMTP_new_album_t() -> *mut LIBMTP_album_t;
	pub fn LIBMTP_destroy_album_t(album: *mut LIBMTP_album_t);
	pub fn LIBMTP_Get_Album_List(device: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_album_t;
	pub fn LIBMTP_Get_Album_List_For_Storage(
		device: *mut LIBMTP_mtpdevice_t,
		storage_id: u32,
	) -> *mut LIBMTP_album_t;
	pub fn LIBMTP_Get_Album(device: *mut LIBMTP_mtpdevice_t, albid: u32) -> *mut LIBMTP_album_t;
	pub fn LIBMTP_Create_New_Album(
		device: *mut LIBMTP_mtpdevice_t,
		metadata: *const LIBMTP_album_t,
	) -> c_int;
	pub fn LIBMTP_Update_Album(
		device: *mut LIBMTP_mtpdevice_t,
		metadata: *const LIBMTP_album_t,
	) -> c_int;
	pub fn LIBMTP_Set_Album_Name(
		device: *mut LIBMTP_mtpdevice_t,
		album: *mut LIBMTP_album_t,
		newname: *const c_char,
	) -> c_int;
	pub fn LIBMTP_Delete_Object(device: *mut LIBMTP_mtpdevice_t, object_id: u32) -> c_int;
	pub fn LIBMTP_Move_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		storage_id: u32,
		parent_id: u32,
	) -> c_int;
	pub fn LIBMTP_Copy_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		storage_id: u32,
		parent_id: u32,
	) -> c_int;
	pub fn LIBMTP_Set_Object_Filename(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		newname: *const c_char,
	) -> c_int;
	pub fn LIBMTP_GetPartialObject(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		offset: u64,
		maxbytes: u32,
		data: *mut *mut c_uchar,
		size: c_uint,
	) -> c_int;
	pub fn LIBMTP_SendPartialObject(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		offset: u64,
		data: *mut c_uchar,
		size: c_uint,
	) -> c_int;
	pub fn LIBMTP_BeginEditObject(device: *mut LIBMTP_mtpdevice_t, id: u32) -> c_int;
	pub fn LIBMTP_EndEditObject(device: *mut LIBMTP_mtpdevice_t, id: u32) -> c_int;
	pub fn LIBMTP_TruncateObject(device: *mut LIBMTP_mtpdevice_t, id: u32, offset: u64) -> c_int;
	pub fn LIBMTP_Read_Event(
		device: *mut LIBMTP_mtpdevice_t,
		event: *mut LIBMTP_event_t,
		out1: *mut u32,
	) -> c_int;
	pub fn LIBMTP_Read_Event_Async(
		device: *mut LIBMTP_mtpdevice_t,
		cb: LIBMTP_event_cb_fn,
		user_data: *mut c_void,
	) -> c_int;
	pub fn LIBMTP_Handle_Events_Timeout_Completed(tv: *mut timeval, completed: *mut c_int)
	-> c_int;
	pub fn LIBMTP_Custom_Operation(
		device: *mut LIBMTP_mtpdevice_t,
		code: u16,
		n_param: c_int,
		...
	) -> c_int;
}
