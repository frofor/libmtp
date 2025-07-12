//! This module provides external FFI bindings to libmtp library.

#![allow(non_camel_case_types)]
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

pub(crate) type LIBMTP_event_t = LIBMTP_event_enum;
pub(crate) type LIBMTP_device_entry_t = LIBMTP_device_entry_struct;
pub(crate) type LIBMTP_raw_device_t = LIBMTP_raw_device_struct;
pub(crate) type LIBMTP_error_t = LIBMTP_error_struct;
pub(crate) type LIBMTP_allowed_values_t = LIBMTP_allowed_values_struct;
pub(crate) type LIBMTP_device_extension_t = LIBMTP_device_extension_struct;
pub(crate) type LIBMTP_mtpdevice_t = LIBMTP_mtpdevice_struct;
pub(crate) type LIBMTP_file_t = LIBMTP_file_struct;
pub(crate) type LIBMTP_track_t = LIBMTP_track_struct;
pub(crate) type LIBMTP_playlist_t = LIBMTP_playlist_struct;
pub(crate) type LIBMTP_album_t = LIBMTP_album_struct;
pub(crate) type LIBMTP_folder_t = LIBMTP_folder_struct;
pub(crate) type LIBMTP_filesampledata_t = LIBMTP_filesampledata_struct;
pub(crate) type LIBMTP_devicestorage_t = LIBMTP_devicestorage_struct;
pub(crate) type LIBMTP_progressfunc_t = Option<unsafe extern "C" fn(u64, u64, *const c_void)>;
pub(crate) type MTPDataGetFunc =
	Option<unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *mut c_uchar, *mut u32) -> u16>;
pub(crate) type MTPDataPutFunc =
	Option<unsafe extern "C" fn(*mut c_void, *mut c_void, u32, *mut c_uchar, *mut u32) -> u16>;
pub(crate) type LIBMTP_event_cb_fn =
	Option<unsafe extern "C" fn(c_int, LIBMTP_event_t, u32, *mut c_void)>;
pub(crate) type PTPErrorFunc = fn(*mut c_void, *const c_char, *mut c_void);
pub(crate) type PTPDebugFunc = fn(*mut c_void, *const c_char, *mut c_void);
pub(crate) type PTPDataGetFunc =
	fn(*mut PTPParams, *mut c_void, c_ulong, *mut c_uchar, *mut c_ulong) -> u16;
pub(crate) type PTPDataPutFunc = fn(*mut PTPParams, *mut c_void, c_ulong, *mut c_uchar) -> u16;
pub(crate) type PTPIOSendReq = fn(*mut PTPParams, *mut PTPContainer, c_int) -> u16;
pub(crate) type PTPIOSendData =
	fn(*mut PTPParams, *mut PTPContainer, u64, *mut PTPDataHandler) -> u16;
pub(crate) type PTPIOGetResp = fn(*mut PTPParams, *mut PTPContainer) -> u16;
pub(crate) type PTPIOGetData = fn(*mut PTPParams, *mut PTPContainer, *mut PTPDataHandler) -> u16;
pub(crate) type PTPIOCancelReq = fn(*mut PTPParams, u32) -> u16;
pub(crate) type PTPIODevStatReq = fn(*mut PTPParams) -> u16;

pub(crate) const LIBMTP_STORAGE_SORTBY_NOTSORTED: c_int = 0;
pub(crate) const LIBMTP_STORAGE_SORTBY_FREESPACE: c_int = 1;
pub(crate) const LIBMTP_STORAGE_SORTBY_MAXSPACE: c_int = 2;
pub(crate) const LIBMTP_FILES_AND_FOLDERS_ROOT: u32 = 4294967295;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub(crate) enum LIBMTP_event_enum {
	#[default]
	None,
	StoreAdded,
	StoreRemoved,
	ObjectAdded,
	ObjectRemoved,
	DevicePropertyChanged,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub(crate) enum LIBMTP_filetype_t {
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
	#[default]
	Unknown,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub(crate) enum LIBMTP_property_t {
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
	#[default]
	Unknown,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum LIBMTP_datatype_t {
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
pub(crate) enum LIBMTP_devicecap_t {
	GetPartialObject,
	SendPartialObject,
	EditObjects,
	MoveObject,
	CopyObject,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub(crate) enum LIBMTP_error_number_t {
	#[default]
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
pub(crate) struct LIBMTP_device_entry_struct {
	pub(crate) vendor: *mut c_char,
	pub(crate) vendor_id: u16,
	pub(crate) product: *mut c_char,
	pub(crate) product_id: u16,
	pub(crate) device_flags: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct LIBMTP_raw_device_struct {
	pub(crate) device_entry: LIBMTP_device_entry_t,
	pub(crate) bus_location: u32,
	pub(crate) devnum: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct LIBMTP_error_struct {
	pub(crate) errornumber: LIBMTP_error_number_t,
	pub(crate) error_text: *mut c_char,
	pub(crate) next: *mut LIBMTP_error_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct LIBMTP_allowed_values_struct {
	pub(crate) u8max: u8,
	pub(crate) u8min: u8,
	pub(crate) u8step: u8,
	pub(crate) u8vals: *mut u8,
	pub(crate) i8max: i8,
	pub(crate) i8min: i8,
	pub(crate) i8step: i8,
	pub(crate) i8vals: *mut i8,
	pub(crate) u16max: u16,
	pub(crate) u16min: u16,
	pub(crate) u16step: u16,
	pub(crate) u16vals: *mut u16,
	pub(crate) i16max: i16,
	pub(crate) i16min: i16,
	pub(crate) i16step: i16,
	pub(crate) i16vals: *mut i16,
	pub(crate) u32max: u32,
	pub(crate) u32min: u32,
	pub(crate) u32step: u32,
	pub(crate) u32vals: *mut u32,
	pub(crate) i32max: i32,
	pub(crate) i32min: i32,
	pub(crate) i32step: i32,
	pub(crate) i32vals: *mut i32,
	pub(crate) u64max: u64,
	pub(crate) u64min: u64,
	pub(crate) u64step: u64,
	pub(crate) u64vals: *mut u64,
	pub(crate) i64max: i64,
	pub(crate) i64min: i64,
	pub(crate) i64step: i64,
	pub(crate) i64vals: *mut i64,
	pub(crate) num_entries: u16,
	pub(crate) datatype: LIBMTP_datatype_t,
	pub(crate) is_range: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct LIBMTP_device_extension_struct {
	pub(crate) name: *mut c_char,
	pub(crate) major: c_int,
	pub(crate) minor: c_int,
	pub(crate) next: *mut LIBMTP_device_extension_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct LIBMTP_mtpdevice_struct {
	pub(crate) object_bitsize: u8,
	pub(crate) params: *mut c_void,
	pub(crate) usbinfo: *mut c_void,
	pub(crate) storage: *mut LIBMTP_devicestorage_t,
	pub(crate) errorstack: *mut LIBMTP_error_t,
	pub(crate) maximum_battery_level: u8,
	pub(crate) default_music_folder: u32,
	pub(crate) default_playlist_folder: u32,
	pub(crate) default_picture_folder: u32,
	pub(crate) default_video_folder: u32,
	pub(crate) default_organizer_folder: u32,
	pub(crate) default_zencast_folder: u32,
	pub(crate) default_album_folder: u32,
	pub(crate) default_text_folder: u32,
	pub(crate) cd: *mut c_void,
	pub(crate) extensions: *mut LIBMTP_device_extension_t,
	pub(crate) cached: c_int,
	pub(crate) next: *mut LIBMTP_mtpdevice_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct LIBMTP_file_struct {
	pub(crate) item_id: u32,
	pub(crate) parent_id: u32,
	pub(crate) storage_id: u32,
	pub(crate) filename: *mut c_char,
	pub(crate) filesize: u64,
	pub(crate) modificationdate: time_t,
	pub(crate) filetype: LIBMTP_filetype_t,
	pub(crate) next: *mut LIBMTP_file_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct LIBMTP_track_struct {
	pub(crate) item_id: u32,
	pub(crate) parent_id: u32,
	pub(crate) storage_id: u32,
	pub(crate) title: *mut c_char,
	pub(crate) artist: *mut c_char,
	pub(crate) composer: *mut c_char,
	pub(crate) genre: *mut c_char,
	pub(crate) album: *mut c_char,
	pub(crate) date: *mut c_char,
	pub(crate) filename: *mut c_char,
	pub(crate) tracknumber: u16,
	pub(crate) duration: u32,
	pub(crate) samplerate: u32,
	pub(crate) nochannels: u16,
	pub(crate) wavecodec: u32,
	pub(crate) bitrate: u32,
	pub(crate) bitratetype: u16,
	pub(crate) rating: u16,
	pub(crate) usecount: u32,
	pub(crate) filesize: u64,
	pub(crate) modificationdate: time_t,
	pub(crate) filetype: LIBMTP_filetype_t,
	pub(crate) next: *mut LIBMTP_track_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct LIBMTP_playlist_struct {
	pub(crate) playlist_id: u32,
	pub(crate) parent_id: u32,
	pub(crate) storage_id: u32,
	pub(crate) name: *mut c_char,
	pub(crate) tracks: *mut u32,
	pub(crate) no_tracks: *mut u32,
	pub(crate) next: *mut LIBMTP_playlist_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct LIBMTP_album_struct {
	pub(crate) album_id: u32,
	pub(crate) parent_id: u32,
	pub(crate) storage_id: u32,
	pub(crate) name: *mut c_char,
	pub(crate) artist: *mut c_char,
	pub(crate) composer: *mut c_char,
	pub(crate) genre: *mut c_char,
	pub(crate) tracks: *mut u32,
	pub(crate) no_tracks: *mut u32,
	pub(crate) next: *mut LIBMTP_album_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct LIBMTP_folder_struct {
	pub(crate) folder_id: u32,
	pub(crate) parent_id: u32,
	pub(crate) storage_id: u32,
	pub(crate) name: *mut c_char,
	pub(crate) sibling: *mut LIBMTP_folder_t,
	pub(crate) child: *mut LIBMTP_folder_t,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct LIBMTP_filesampledata_struct {
	pub(crate) width: u32,
	pub(crate) height: u32,
	pub(crate) duration: u32,
	pub(crate) filetype: LIBMTP_filetype_t,
	pub(crate) size: u64,
	pub(crate) data: *mut c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct LIBMTP_devicestorage_struct {
	pub(crate) id: u32,
	pub(crate) StorageType: u16,
	pub(crate) FilesystemType: u16,
	pub(crate) AccessCapability: u16,
	pub(crate) MaxCapacity: u64,
	pub(crate) FreeSpaceInBytes: u64,
	pub(crate) FreeSpaceInObjects: u64,
	pub(crate) StorageDescription: *mut c_char,
	pub(crate) VolumeIdentifier: *mut c_char,
	pub(crate) next: *mut LIBMTP_devicestorage_t,
	pub(crate) prev: *mut LIBMTP_devicestorage_t,
}

#[repr(C)]
#[derive(Clone)]
pub(crate) struct PTPParams {
	pub(crate) device_flags: u32,
	pub(crate) byteorder: u8,
	pub(crate) maxpacketsize: u16,
	pub(crate) sendreq_func: PTPIOSendReq,
	pub(crate) senddata_func: PTPIOSendData,
	pub(crate) getresp_func: PTPIOGetResp,
	pub(crate) getdata_func: PTPIOGetData,
	pub(crate) event_check: PTPIOGetResp,
	pub(crate) event_check_queue: PTPIOGetResp,
	pub(crate) event_wait: PTPIOGetResp,
	pub(crate) cancelreq_func: PTPIOCancelReq,
	pub(crate) devstatreq_func: PTPIODevStatReq,
	pub(crate) error_func: PTPErrorFunc,
	pub(crate) debug_func: PTPDebugFunc,
	pub(crate) data: *mut c_void,
	pub(crate) transaction_id: u32,
	pub(crate) session_id: u32,
	pub(crate) opencapture_transid: u32,
	pub(crate) split_header_data: c_int,
	pub(crate) ocs64: c_int,
	pub(crate) nrofobjectformats: c_int,
	pub(crate) object_formats: *mut MTPObjectFormat,
	pub(crate) objects: *mut PTPObject,
	pub(crate) nrofobjects: c_uint,
	pub(crate) deviceinfo: PTPDeviceInfo,
	pub(crate) events: *mut PTPContainer,
	pub(crate) nrofevents: c_uint,
	pub(crate) capcnt: c_uint,
	pub(crate) inlineview: c_int,
	pub(crate) cachetime: c_int,
	pub(crate) storageids: PTPStorageIDs,
	pub(crate) storagechanged: c_int,
	pub(crate) deviceproperties: *mut PTPDeviceProperty,
	pub(crate) nrofdeviceproperties: c_uint,
	pub(crate) canon_proprs: *mut PTPCanon_Property,
	pub(crate) nrofcanon_props: c_uint,
	pub(crate) canon_viewfinder_on: c_int,
	pub(crate) canon_event_mode: c_int,
	pub(crate) backlogentries: *mut PTPCanon_changes_entry,
	pub(crate) nrofbacklogentries: c_uint,
	pub(crate) eos_captureenabled: c_int,
	pub(crate) eos_camerastatus: c_int,
	pub(crate) controlmode: c_int,
	pub(crate) event90c7works: c_int,
	pub(crate) deletesdramfails: c_int,
	pub(crate) starttime: timeval,
	pub(crate) wifi_profiles_version: u8,
	pub(crate) wifi_profiles_number: u8,
	pub(crate) wifi_profiles: *mut PTPNIKONWifiProfile,
	pub(crate) cmdfd: c_int,
	pub(crate) evtfd: c_int,
	pub(crate) jpgfd: c_int,
	pub(crate) cameraguid: [u8; 16],
	pub(crate) eventpipeid: u32,
	pub(crate) cameraname: *mut c_char,
	pub(crate) outer_deviceinfo: PTPDeviceInfo,
	pub(crate) olympus_cmd: *mut c_char,
	pub(crate) olympus_reply: *mut c_char,
	pub(crate) outer_params: Box<PTPParams>,
	pub(crate) response_packet: *mut u8,
	pub(crate) response_packet_size: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct PTPDeviceInfo {
	pub(crate) StandardVersion: u16,
	pub(crate) VendorExtensionID: u32,
	pub(crate) VendorExtensionDesc: *mut c_char,
	pub(crate) FunctionalMode: u16,
	pub(crate) OperationsSupported_len: u32,
	pub(crate) OperationsSupported: *mut u16,
	pub(crate) EventsSupported_len: u32,
	pub(crate) EventsSupported: *mut u16,
	pub(crate) DevicePropertiesSupported_len: u32,
	pub(crate) DevicePropertiesSupported: *mut u16,
	pub(crate) CaptureFormats_len: u32,
	pub(crate) CaptureFormats: *mut u16,
	pub(crate) ImageFormats_len: u32,
	pub(crate) ImageFormats: *mut u16,
	pub(crate) Manufacturer: *mut c_char,
	pub(crate) Model: *mut c_char,
	pub(crate) DeviceVersion: *mut c_char,
	pub(crate) SerialNumber: *mut c_char,
}

#[repr(C)]
pub(crate) struct PTPDeviceProperty {
	pub(crate) timestamp: time_t,
	pub(crate) desc: PTPDevicePropDesc,
	pub(crate) value: PTPPropertyValue,
}

#[repr(C)]
pub(crate) struct PTPDevicePropDesc {
	pub(crate) DevicePropertyCode: u16,
	pub(crate) DataType: u16,
	pub(crate) GetSet: u8,
	pub(crate) FactoryDefaultValue: PTPPropertyValue,
	pub(crate) CurrentValue: PTPPropertyValue,
	pub(crate) FormFlag: u8,
	pub(crate) FORM: PTPDevicePropDescForm,
}

#[repr(C)]
pub(crate) union PTPDevicePropDescForm {
	pub(crate) Enum: PTPPropDescEnumForm,
	pub(crate) Range: ManuallyDrop<PTPPropDescRangeForm>,
}

#[repr(C)]
pub(crate) struct PTPCanon_Property {
	pub(crate) size: u32,
	pub(crate) proptype: u32,
	pub(crate) data: *mut c_uchar,
	pub(crate) dpd: PTPDevicePropDesc,
}

#[repr(C)]
pub(crate) struct PTPCanon_changes_entry {
	pub(crate) r#type: PTPCanon_changes_types,
	pub(crate) u: PTPCanon_changes_entryUnion,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) enum PTPCanon_changes_types {
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
pub(crate) union PTPCanon_changes_entryUnion {
	pub(crate) object: PTPCanon_New_Object,
	pub(crate) info: *mut c_char,
	pub(crate) propid: u16,
	pub(crate) status: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct PTPCanon_New_Object {
	pub(crate) oid: u32,
	pub(crate) oi: PTPObjectInfo,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct PTPObjectInfo {
	pub(crate) StorageID: u32,
	pub(crate) ObjectFormat: u16,
	pub(crate) ProtectionStatus: u16,
	pub(crate) ObjectCompressedSize: u64,
	pub(crate) ThumbFormat: u16,
	pub(crate) ThumbCompressedSize: u32,
	pub(crate) ThumbPixWidth: u32,
	pub(crate) ThumbPixHeight: u32,
	pub(crate) ImagePixWidth: u32,
	pub(crate) ImagePixHeight: u32,
	pub(crate) ImageBitDepth: u32,
	pub(crate) ParentObject: u32,
	pub(crate) AssociationType: u16,
	pub(crate) AssociationDesc: u32,
	pub(crate) SequenceNumber: u32,
	pub(crate) Filename: *mut c_char,
	pub(crate) CaptureDate: time_t,
	pub(crate) ModificationDate: time_t,
	pub(crate) Keywords: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub(crate) struct PTPNIKONWifiProfile {
	pub(crate) profile_name: [c_char; 17],
	pub(crate) device_type: u8,
	pub(crate) icon_type: u8,
	pub(crate) essid: [c_char; 33],
	pub(crate) id: u8,
	pub(crate) valid: u8,
	pub(crate) display_order: u8,
	pub(crate) creation_date: [c_char; 16],
	pub(crate) lastusage_date: [c_char; 16],
	pub(crate) ip_address: u32,
	pub(crate) subnet_mask: u8,
	pub(crate) gateway_address: u32,
	pub(crate) address_mode: u8,
	pub(crate) access_mode: u8,
	pub(crate) wifi_channel: u8,
	pub(crate) authentification: u8,
	pub(crate) encryption: u8,
	pub(crate) key: [u8; 64],
	pub(crate) key_nr: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct PTPContainer {
	pub(crate) Code: u16,
	pub(crate) SessionID: u32,
	pub(crate) Transaction_ID: u32,
	pub(crate) Param1: u32,
	pub(crate) Param2: u32,
	pub(crate) Param3: u32,
	pub(crate) Param4: u32,
	pub(crate) Param5: u32,
	pub(crate) Nparam: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct MTPObjectFormat {
	pub(crate) ofc: u16,
	pub(crate) nrofpds: c_uint,
	pub(crate) pds: *mut MTPPropertyDesc,
}

#[repr(C)]
pub(crate) struct MTPPropertyDesc {
	pub(crate) opc: u16,
	pub(crate) opd: PTPObjectPropDesc,
}

#[repr(C)]
pub(crate) struct PTPObjectPropDesc {
	pub(crate) ObjectPropertyCode: u16,
	pub(crate) DataType: u16,
	pub(crate) GetSet: u8,
	pub(crate) FactoryDefaultValue: PTPPropertyValue,
	pub(crate) GroupCode: u32,
	pub(crate) FormFlag: u8,
	pub(crate) FORM: PTPObjectPropDescForm,
}

#[repr(C)]
pub(crate) union PTPObjectPropDescForm {
	pub(crate) Enum: PTPPropDescEnumForm,
	pub(crate) Range: ManuallyDrop<PTPPropDescRangeForm>,
	pub(crate) DateTime: PTPPropDescStringForm,
	pub(crate) FixedLengthArray: PTPPropDescArrayLengthForm,
	pub(crate) RegularExpression: PTPPropDescStringForm,
	pub(crate) ByteArray: PTPPropDescArrayLengthForm,
	pub(crate) LongString: PTPPropDescStringForm,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct PTPPropDescEnumForm {
	pub(crate) NumberOfValues: u16,
	pub(crate) SupportedValues: *mut PTPPropertyValue,
}

#[repr(C)]
pub(crate) struct PTPPropDescRangeForm {
	pub(crate) MinimumValue: PTPPropertyValue,
	pub(crate) MaximumValue: PTPPropertyValue,
	pub(crate) StepSize: PTPPropertyValue,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct PTPPropDescStringForm {
	pub(crate) Strict: *mut c_char,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct PTPPropDescArrayLengthForm {
	pub(crate) NumberOfValues: u16,
}

#[repr(C)]
pub(crate) union PTPPropertyValue {
	pub(crate) str: *mut c_char,
	pub(crate) u8: u8,
	pub(crate) i8: i8,
	pub(crate) u16: u16,
	pub(crate) i16: i16,
	pub(crate) u32: u32,
	pub(crate) i32: i32,
	pub(crate) u64: u64,
	pub(crate) i64: i64,
	pub(crate) a: ManuallyDrop<Box<PTPPropertyValueArray>>,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct PTPObject {
	pub(crate) oid: u32,
	pub(crate) flags: c_uint,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct PTPStorageIDs {
	pub(crate) n: u32,
	pub(crate) Storage: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct PTPPropertyValueArray {
	pub(crate) count: u32,
	pub(crate) v: *mut PTPPropertyValue,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, Debug)]
pub(crate) struct PTPDataHandler {
	pub(crate) getfunc: PTPDataGetFunc,
	pub(crate) putfunc: PTPDataPutFunc,
	pub(crate) r#priv: *mut c_void,
}

#[link(name = "mtp")]
unsafe extern "C" {
	pub(crate) static mut LIBMTP_debug: c_int;
	pub(crate) fn LIBMTP_Set_Debug(level: c_int);
	pub(crate) fn LIBMTP_Init();
	pub(crate) fn LIBMTP_Get_Supported_Devices_List(
		devices: *mut *mut LIBMTP_device_entry_t,
		numdevs: *mut c_int,
	) -> i32;
	pub(crate) fn LIBMTP_Detect_Raw_Devices(
		devices: *mut *mut LIBMTP_raw_device_t,
		numdevs: *mut c_int,
	) -> LIBMTP_error_number_t;
	pub(crate) fn LIBMTP_Check_Specific_Device(busno: c_int, devno: c_int) -> c_int;
	pub(crate) fn LIBMTP_Open_Raw_Device(
		rawdevice: *mut LIBMTP_raw_device_t,
	) -> *mut LIBMTP_mtpdevice_t;
	pub(crate) fn LIBMTP_Open_Raw_Device_Uncached(
		rawdevice: *mut LIBMTP_raw_device_t,
	) -> *mut LIBMTP_mtpdevice_t;
	pub(crate) fn LIBMTP_Get_Device(device_nr: c_int) -> *mut LIBMTP_mtpdevice_t;
	pub(crate) fn LIBMTP_Get_First_Device() -> *mut LIBMTP_mtpdevice_t;
	pub(crate) fn LIBMTP_Get_Device_By_SerialNumber(
		serial_number: *const c_char,
	) -> *mut LIBMTP_mtpdevice_t;
	pub(crate) fn LIBMTP_Get_Device_By_ID(device_id: *const c_char) -> *mut LIBMTP_mtpdevice_t;
	pub(crate) fn LIBMTP_Get_Connected_Devices(
		device_list: *mut *mut LIBMTP_mtpdevice_t,
	) -> LIBMTP_error_number_t;
	pub(crate) fn LIBMTP_Number_Devices_In_List(device_list: *mut LIBMTP_mtpdevice_t) -> u32;
	pub(crate) fn LIBMTP_Release_Device_List(device: *mut LIBMTP_mtpdevice_t);
	pub(crate) fn LIBMTP_Release_Device(device: *mut LIBMTP_mtpdevice_t);
	pub(crate) fn LIBMTP_Dump_Device_Info(device: *mut LIBMTP_mtpdevice_t);
	pub(crate) fn LIBMTP_Reset_Device(device: *mut LIBMTP_mtpdevice_t) -> c_int;
	pub(crate) fn LIBMTP_Get_Manufacturername(device: *mut LIBMTP_mtpdevice_t) -> *mut c_char;
	pub(crate) fn LIBMTP_Get_Modelname(device: *mut LIBMTP_mtpdevice_t) -> *mut c_char;
	pub(crate) fn LIBMTP_Get_Serialnumber(device: *mut LIBMTP_mtpdevice_t) -> *mut c_char;
	pub(crate) fn LIBMTP_Get_Deviceversion(device: *mut LIBMTP_mtpdevice_t) -> *mut c_char;
	pub(crate) fn LIBMTP_Get_Friendlyname(device: *mut LIBMTP_mtpdevice_t) -> *mut c_char;
	pub(crate) fn LIBMTP_Set_Friendlyname(
		device: *mut LIBMTP_mtpdevice_t,
		friendlyname: *const c_char,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_Syncpartner(device: *mut LIBMTP_mtpdevice_t) -> *mut c_char;
	pub(crate) fn LIBMTP_Set_Syncpartner(
		device: *mut LIBMTP_mtpdevice_t,
		syncpartner: *const c_char,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_Batterylevel(
		device: *mut LIBMTP_mtpdevice_t,
		maximum_level: *mut u8,
		current_level: *mut u8,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_Secure_Time(
		device: *mut LIBMTP_mtpdevice_t,
		sectime: *mut *mut c_char,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_Device_Certificate(
		device: *mut LIBMTP_mtpdevice_t,
		devcert: *mut *mut c_char,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_Supported_Filetypes(
		device: *mut LIBMTP_mtpdevice_t,
		filetypes: *mut *mut u16,
		length: *mut u16,
	) -> c_int;
	pub(crate) fn LIBMTP_Check_Capability(
		device: *mut LIBMTP_mtpdevice_t,
		cap: LIBMTP_devicecap_t,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_Errorstack(device: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_error_t;
	pub(crate) fn LIBMTP_Clear_Errorstack(device: *mut LIBMTP_mtpdevice_t);
	pub(crate) fn LIBMTP_Dump_Errorstack(device: *mut LIBMTP_mtpdevice_t);
	pub(crate) fn LIBMTP_FreeMemory(mem: *mut c_void);
	pub(crate) fn LIBMTP_Get_Storage(device: *mut LIBMTP_mtpdevice_t, sortby: c_int) -> c_int;
	pub(crate) fn LIBMTP_Format_Storage(
		device: *mut LIBMTP_mtpdevice_t,
		storage: *mut LIBMTP_devicestorage_t,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_String_From_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
	) -> *mut c_char;
	pub(crate) fn LIBMTP_Get_u64_From_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value_default: u64,
	) -> u64;
	pub(crate) fn LIBMTP_Get_u32_From_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value_default: u32,
	) -> u32;
	pub(crate) fn LIBMTP_Get_u16_From_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value_default: u16,
	) -> u16;
	pub(crate) fn LIBMTP_Get_u8_From_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value_default: u8,
	) -> u8;
	pub(crate) fn LIBMTP_Set_Object_String(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		string: *const c_char,
	) -> c_int;
	pub(crate) fn LIBMTP_Set_Object_u32(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value: u32,
	) -> c_int;
	pub(crate) fn LIBMTP_Set_Object_u16(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value: u16,
	) -> c_int;
	pub(crate) fn LIBMTP_Set_Object_u8(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		attribute_id: LIBMTP_property_t,
		value: u8,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_Property_Description(inproperty: LIBMTP_property_t) -> *const c_char;
	pub(crate) fn LIBMTP_Is_Property_Supported(
		device: *mut LIBMTP_mtpdevice_t,
		property: LIBMTP_property_t,
		filetype: LIBMTP_filetype_t,
	) -> c_int;
	pub(crate) fn LIBTP_Get_Allowed_Property_Values(
		device: *mut LIBMTP_mtpdevice_t,
		property: LIBMTP_property_t,
		filetype: LIBMTP_filetype_t,
		allowed_vals: *mut LIBMTP_allowed_values_t,
	) -> c_int;
	pub(crate) fn LIBMTP_destroy_allowed_values_t(allowed_vals: *mut LIBMTP_allowed_values_t);
	pub(crate) fn LIBMTP_new_file_t() -> *mut LIBMTP_file_t;
	pub(crate) fn LIBMTP_destroy_file_t(file: *mut LIBMTP_file_t);
	pub(crate) fn LIBMTP_Get_Filetype_Description(intype: LIBMTP_filetype_t) -> *const c_char;
	pub(crate) fn LIBMTP_Get_Filelisting(device: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_file_t;
	pub(crate) fn LIBMTP_Get_Filelisting_With_Callback(
		device: *mut LIBMTP_mtpdevice_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> *mut LIBMTP_file_t;
	pub(crate) fn LIBMTP_Get_Files_And_Folders(
		device: *mut LIBMTP_mtpdevice_t,
		storage: u32,
		parent: u32,
	) -> *mut LIBMTP_file_t;
	pub(crate) fn LIBMTP_Get_Children(
		device: *mut LIBMTP_mtpdevice_t,
		storage: u32,
		parent: u32,
		out: *mut *mut u32,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_Filemetadata(
		device: *mut LIBMTP_mtpdevice_t,
		fileid: u32,
	) -> *mut LIBMTP_file_t;
	pub(crate) fn LIBMTP_Get_File_To_File(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		path: *const c_char,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_File_To_File_Descriptor(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		fd: c_int,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_File_To_Handler(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		put_func: MTPDataPutFunc,
		r#priv: *mut c_void,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Send_File_From_File(
		device: *mut LIBMTP_mtpdevice_t,
		path: *const c_char,
		filedata: *mut LIBMTP_file_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Send_File_From_File_Descriptor(
		device: *mut LIBMTP_mtpdevice_t,
		fd: c_int,
		filedata: *mut LIBMTP_file_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Send_File_From_Handler(
		device: *mut LIBMTP_mtpdevice_t,
		get_func: MTPDataPutFunc,
		r#priv: *mut c_void,
		filedata: *mut LIBMTP_file_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Set_File_Name(
		device: *mut LIBMTP_mtpdevice_t,
		file: *mut LIBMTP_file_t,
		newname: *const c_char,
	) -> c_int;
	pub(crate) fn LIBMTP_new_filesampledata_t() -> *mut LIBMTP_filesampledata_t;
	pub(crate) fn LIBMTP_destroy_filesampledata_t(sample: *mut LIBMTP_filesampledata_t);
	pub(crate) fn LIBMTP_Get_Representative_Sample_Format(
		device: *mut LIBMTP_mtpdevice_t,
		filetype: LIBMTP_filetype_t,
		sample: *mut *mut LIBMTP_filesampledata_t,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_Thumbnail(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		data: *mut *mut c_uchar,
		size: *mut c_uint,
	) -> c_int;
	pub(crate) fn LIBMTP_new_track_t() -> *mut LIBMTP_track_t;
	pub(crate) fn LIBMTP_destroy_track_t(track: *mut LIBMTP_track_t);
	pub(crate) fn LIBMTP_Get_Tracklisting(device: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_track_t;
	pub(crate) fn LIBMTP_Get_Tracklisting_With_Callback(
		device: *mut LIBMTP_mtpdevice_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> *mut LIBMTP_track_t;
	pub(crate) fn LIBMTP_Get_Tracklisting_With_Callback_For_Storage(
		device: *mut LIBMTP_mtpdevice_t,
		storage_id: u32,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> *mut LIBMTP_track_t;
	pub(crate) fn LIBMTP_Get_Trackmetadata(
		device: *mut LIBMTP_mtpdevice_t,
		trackid: u32,
	) -> *mut LIBMTP_track_t;
	pub(crate) fn LIBMTP_Get_Track_To_File(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		path: *const c_char,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_Track_To_File_Descriptor(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		fd: c_int,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Get_Track_To_Handler(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		put_func: MTPDataPutFunc,
		r#priv: *mut c_void,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Send_Track_From_File(
		device: *mut LIBMTP_mtpdevice_t,
		a: *const c_char,
		track: *const LIBMTP_track_t,
		progress: LIBMTP_progressfunc_t,
		a: *const c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Send_Track_From_File_Descriptor(
		device: *mut LIBMTP_mtpdevice_t,
		path: *const c_char,
		metadata: *const LIBMTP_track_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Send_Track_From_Handler(
		device: *mut LIBMTP_mtpdevice_t,
		get_func: MTPDataGetFunc,
		r#priv: *mut c_void,
		metadata: *const LIBMTP_track_t,
		callback: LIBMTP_progressfunc_t,
		data: *const c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Update_Track_Metadata(
		device: *mut LIBMTP_mtpdevice_t,
		metadata: *const LIBMTP_track_t,
	) -> c_int;
	pub(crate) fn LIBMTP_Track_Exists(device: *mut LIBMTP_mtpdevice_t, id: u32) -> c_int;
	pub(crate) fn LIBMTP_Set_Track_Name(
		device: *mut LIBMTP_mtpdevice_t,
		track: *mut LIBMTP_track_t,
		newname: *const c_char,
	) -> c_int;
	pub(crate) fn LIBMTP_new_folder_t() -> *mut LIBMTP_folder_t;
	pub(crate) fn LIBMTP_destroy_folder_t(folder: *mut LIBMTP_folder_t);
	pub(crate) fn LIBMTP_Get_Folder_List(device: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_folder_t;
	pub(crate) fn LIBMTP_Get_Folder_List_For_Storage(
		device: *mut LIBMTP_mtpdevice_t,
		storage: u32,
	) -> *mut LIBMTP_folder_t;
	pub(crate) fn LIBMTP_Find_Folder(
		folderlist: *mut LIBMTP_folder_t,
		id: u32,
	) -> *mut LIBMTP_folder_t;
	pub(crate) fn LIBMTP_Create_Folder(
		device: *mut LIBMTP_mtpdevice_t,
		name: *mut c_char,
		parent_id: u32,
		storage_id: u32,
	) -> u32;
	pub(crate) fn LIBMTP_Set_Folder_Name(
		device: *mut LIBMTP_mtpdevice_t,
		folder: *mut LIBMTP_folder_t,
		newname: *const c_char,
	) -> c_int;
	pub(crate) fn LIBMTP_new_playlist_t() -> *mut LIBMTP_playlist_t;
	pub(crate) fn LIBMTP_destroy_playlist_t(playlist: *mut LIBMTP_playlist_t);
	pub(crate) fn LIBMTP_Get_Playlist_List(
		device: *mut LIBMTP_mtpdevice_t,
	) -> *mut LIBMTP_playlist_t;
	pub(crate) fn LIBMTP_Get_Playlist(
		device: *mut LIBMTP_mtpdevice_t,
		plid: u32,
	) -> *mut LIBMTP_playlist_t;
	pub(crate) fn LIBMTP_Create_New_Playlist(
		device: *mut LIBMTP_mtpdevice_t,
		metadata: *const LIBMTP_playlist_t,
	) -> c_int;
	pub(crate) fn LIBMTP_Update_Playlist(
		device: *mut LIBMTP_mtpdevice_t,
		metadata: *const LIBMTP_playlist_t,
	) -> c_int;
	pub(crate) fn LIBMTP_Set_Playlist_Name(
		device: *mut LIBMTP_mtpdevice_t,
		playlist: *mut LIBMTP_playlist_t,
		newname: *const c_char,
	) -> c_int;
	pub(crate) fn LIBMTP_new_album_t() -> *mut LIBMTP_album_t;
	pub(crate) fn LIBMTP_destroy_album_t(album: *mut LIBMTP_album_t);
	pub(crate) fn LIBMTP_Get_Album_List(device: *mut LIBMTP_mtpdevice_t) -> *mut LIBMTP_album_t;
	pub(crate) fn LIBMTP_Get_Album_List_For_Storage(
		device: *mut LIBMTP_mtpdevice_t,
		storage_id: u32,
	) -> *mut LIBMTP_album_t;
	pub(crate) fn LIBMTP_Get_Album(
		device: *mut LIBMTP_mtpdevice_t,
		albid: u32,
	) -> *mut LIBMTP_album_t;
	pub(crate) fn LIBMTP_Create_New_Album(
		device: *mut LIBMTP_mtpdevice_t,
		metadata: *const LIBMTP_album_t,
	) -> c_int;
	pub(crate) fn LIBMTP_Update_Album(
		device: *mut LIBMTP_mtpdevice_t,
		metadata: *const LIBMTP_album_t,
	) -> c_int;
	pub(crate) fn LIBMTP_Set_Album_Name(
		device: *mut LIBMTP_mtpdevice_t,
		album: *mut LIBMTP_album_t,
		newname: *const c_char,
	) -> c_int;
	pub(crate) fn LIBMTP_Delete_Object(device: *mut LIBMTP_mtpdevice_t, object_id: u32) -> c_int;
	pub(crate) fn LIBMTP_Move_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		storage_id: u32,
		parent_id: u32,
	) -> c_int;
	pub(crate) fn LIBMTP_Copy_Object(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		storage_id: u32,
		parent_id: u32,
	) -> c_int;
	pub(crate) fn LIBMTP_Set_Object_Filename(
		device: *mut LIBMTP_mtpdevice_t,
		object_id: u32,
		newname: *const c_char,
	) -> c_int;
	pub(crate) fn LIBMTP_GetPartialObject(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		offset: u64,
		maxbytes: u32,
		data: *mut *mut c_uchar,
		size: c_uint,
	) -> c_int;
	pub(crate) fn LIBMTP_SendPartialObject(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		offset: u64,
		data: *mut c_uchar,
		size: c_uint,
	) -> c_int;
	pub(crate) fn LIBMTP_BeginEditObject(device: *mut LIBMTP_mtpdevice_t, id: u32) -> c_int;
	pub(crate) fn LIBMTP_EndEditObject(device: *mut LIBMTP_mtpdevice_t, id: u32) -> c_int;
	pub(crate) fn LIBMTP_TruncateObject(
		device: *mut LIBMTP_mtpdevice_t,
		id: u32,
		offset: u64,
	) -> c_int;
	pub(crate) fn LIBMTP_Read_Event(
		device: *mut LIBMTP_mtpdevice_t,
		event: *mut LIBMTP_event_t,
		out1: *mut u32,
	) -> c_int;
	pub(crate) fn LIBMTP_Read_Event_Async(
		device: *mut LIBMTP_mtpdevice_t,
		cb: LIBMTP_event_cb_fn,
		user_data: *mut c_void,
	) -> c_int;
	pub(crate) fn LIBMTP_Handle_Events_Timeout_Completed(
		tv: *mut timeval,
		completed: *mut c_int,
	) -> c_int;
	pub(crate) fn LIBMTP_Custom_Operation(
		device: *mut LIBMTP_mtpdevice_t,
		code: u16,
		n_param: c_int,
		...
	) -> c_int;
}
