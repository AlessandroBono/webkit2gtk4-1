// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use glib::translate::*;
#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
use glib::value::FromValue;
#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
use glib::value::ToValue;
#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
use glib::StaticType;
#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
use glib::Type;
use std::fmt;

#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WebKitConsoleMessageLevel")]
pub enum ConsoleMessageLevel {
    #[doc(alias = "WEBKIT_CONSOLE_MESSAGE_LEVEL_INFO")]
    Info,
    #[doc(alias = "WEBKIT_CONSOLE_MESSAGE_LEVEL_LOG")]
    Log,
    #[doc(alias = "WEBKIT_CONSOLE_MESSAGE_LEVEL_WARNING")]
    Warning,
    #[doc(alias = "WEBKIT_CONSOLE_MESSAGE_LEVEL_ERROR")]
    Error,
    #[doc(alias = "WEBKIT_CONSOLE_MESSAGE_LEVEL_DEBUG")]
    Debug,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
impl fmt::Display for ConsoleMessageLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ConsoleMessageLevel::{}",
            match *self {
                Self::Info => "Info",
                Self::Log => "Log",
                Self::Warning => "Warning",
                Self::Error => "Error",
                Self::Debug => "Debug",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
#[doc(hidden)]
impl IntoGlib for ConsoleMessageLevel {
    type GlibType = ffi::WebKitConsoleMessageLevel;

    fn into_glib(self) -> ffi::WebKitConsoleMessageLevel {
        match self {
            Self::Info => ffi::WEBKIT_CONSOLE_MESSAGE_LEVEL_INFO,
            Self::Log => ffi::WEBKIT_CONSOLE_MESSAGE_LEVEL_LOG,
            Self::Warning => ffi::WEBKIT_CONSOLE_MESSAGE_LEVEL_WARNING,
            Self::Error => ffi::WEBKIT_CONSOLE_MESSAGE_LEVEL_ERROR,
            Self::Debug => ffi::WEBKIT_CONSOLE_MESSAGE_LEVEL_DEBUG,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
#[doc(hidden)]
impl FromGlib<ffi::WebKitConsoleMessageLevel> for ConsoleMessageLevel {
    unsafe fn from_glib(value: ffi::WebKitConsoleMessageLevel) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::WEBKIT_CONSOLE_MESSAGE_LEVEL_INFO => Self::Info,
            ffi::WEBKIT_CONSOLE_MESSAGE_LEVEL_LOG => Self::Log,
            ffi::WEBKIT_CONSOLE_MESSAGE_LEVEL_WARNING => Self::Warning,
            ffi::WEBKIT_CONSOLE_MESSAGE_LEVEL_ERROR => Self::Error,
            ffi::WEBKIT_CONSOLE_MESSAGE_LEVEL_DEBUG => Self::Debug,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
impl StaticType for ConsoleMessageLevel {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::webkit_console_message_level_get_type()) }
    }
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
impl glib::value::ValueType for ConsoleMessageLevel {
    type Type = Self;
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
unsafe impl<'a> FromValue<'a> for ConsoleMessageLevel {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
impl ToValue for ConsoleMessageLevel {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WebKitConsoleMessageSource")]
pub enum ConsoleMessageSource {
    #[doc(alias = "WEBKIT_CONSOLE_MESSAGE_SOURCE_JAVASCRIPT")]
    Javascript,
    #[doc(alias = "WEBKIT_CONSOLE_MESSAGE_SOURCE_NETWORK")]
    Network,
    #[doc(alias = "WEBKIT_CONSOLE_MESSAGE_SOURCE_CONSOLE_API")]
    ConsoleApi,
    #[doc(alias = "WEBKIT_CONSOLE_MESSAGE_SOURCE_SECURITY")]
    Security,
    #[doc(alias = "WEBKIT_CONSOLE_MESSAGE_SOURCE_OTHER")]
    Other,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
impl fmt::Display for ConsoleMessageSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ConsoleMessageSource::{}",
            match *self {
                Self::Javascript => "Javascript",
                Self::Network => "Network",
                Self::ConsoleApi => "ConsoleApi",
                Self::Security => "Security",
                Self::Other => "Other",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
#[doc(hidden)]
impl IntoGlib for ConsoleMessageSource {
    type GlibType = ffi::WebKitConsoleMessageSource;

    fn into_glib(self) -> ffi::WebKitConsoleMessageSource {
        match self {
            Self::Javascript => ffi::WEBKIT_CONSOLE_MESSAGE_SOURCE_JAVASCRIPT,
            Self::Network => ffi::WEBKIT_CONSOLE_MESSAGE_SOURCE_NETWORK,
            Self::ConsoleApi => ffi::WEBKIT_CONSOLE_MESSAGE_SOURCE_CONSOLE_API,
            Self::Security => ffi::WEBKIT_CONSOLE_MESSAGE_SOURCE_SECURITY,
            Self::Other => ffi::WEBKIT_CONSOLE_MESSAGE_SOURCE_OTHER,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
#[doc(hidden)]
impl FromGlib<ffi::WebKitConsoleMessageSource> for ConsoleMessageSource {
    unsafe fn from_glib(value: ffi::WebKitConsoleMessageSource) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::WEBKIT_CONSOLE_MESSAGE_SOURCE_JAVASCRIPT => Self::Javascript,
            ffi::WEBKIT_CONSOLE_MESSAGE_SOURCE_NETWORK => Self::Network,
            ffi::WEBKIT_CONSOLE_MESSAGE_SOURCE_CONSOLE_API => Self::ConsoleApi,
            ffi::WEBKIT_CONSOLE_MESSAGE_SOURCE_SECURITY => Self::Security,
            ffi::WEBKIT_CONSOLE_MESSAGE_SOURCE_OTHER => Self::Other,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
impl StaticType for ConsoleMessageSource {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::webkit_console_message_source_get_type()) }
    }
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
impl glib::value::ValueType for ConsoleMessageSource {
    type Type = Self;
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
unsafe impl<'a> FromValue<'a> for ConsoleMessageSource {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_12", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_12")))]
impl ToValue for ConsoleMessageSource {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WebKitContextMenuAction")]
pub enum ContextMenuAction {
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_NO_ACTION")]
    NoAction,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK")]
    OpenLink,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK_IN_NEW_WINDOW")]
    OpenLinkInNewWindow,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_LINK_TO_DISK")]
    DownloadLinkToDisk,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_COPY_LINK_TO_CLIPBOARD")]
    CopyLinkToClipboard,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_OPEN_IMAGE_IN_NEW_WINDOW")]
    OpenImageInNewWindow,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_IMAGE_TO_DISK")]
    DownloadImageToDisk,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_TO_CLIPBOARD")]
    CopyImageToClipboard,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_URL_TO_CLIPBOARD")]
    CopyImageUrlToClipboard,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_OPEN_FRAME_IN_NEW_WINDOW")]
    OpenFrameInNewWindow,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_GO_BACK")]
    GoBack,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_GO_FORWARD")]
    GoForward,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_STOP")]
    Stop,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_RELOAD")]
    Reload,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_COPY")]
    Copy,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_CUT")]
    Cut,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_PASTE")]
    Paste,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_DELETE")]
    Delete,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_SELECT_ALL")]
    SelectAll,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_INPUT_METHODS")]
    InputMethods,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_UNICODE")]
    Unicode,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_SPELLING_GUESS")]
    SpellingGuess,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_NO_GUESSES_FOUND")]
    NoGuessesFound,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_IGNORE_SPELLING")]
    IgnoreSpelling,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_LEARN_SPELLING")]
    LearnSpelling,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_IGNORE_GRAMMAR")]
    IgnoreGrammar,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_FONT_MENU")]
    FontMenu,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_BOLD")]
    Bold,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_ITALIC")]
    Italic,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_UNDERLINE")]
    Underline,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_OUTLINE")]
    Outline,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_INSPECT_ELEMENT")]
    InspectElement,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_OPEN_VIDEO_IN_NEW_WINDOW")]
    OpenVideoInNewWindow,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_OPEN_AUDIO_IN_NEW_WINDOW")]
    OpenAudioInNewWindow,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_COPY_VIDEO_LINK_TO_CLIPBOARD")]
    CopyVideoLinkToClipboard,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_COPY_AUDIO_LINK_TO_CLIPBOARD")]
    CopyAudioLinkToClipboard,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_CONTROLS")]
    ToggleMediaControls,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_LOOP")]
    ToggleMediaLoop,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_ENTER_VIDEO_FULLSCREEN")]
    EnterVideoFullscreen,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PLAY")]
    MediaPlay,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PAUSE")]
    MediaPause,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_MEDIA_MUTE")]
    MediaMute,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_VIDEO_TO_DISK")]
    DownloadVideoToDisk,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_AUDIO_TO_DISK")]
    DownloadAudioToDisk,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_INSERT_EMOJI")]
    InsertEmoji,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_PASTE_AS_PLAIN_TEXT")]
    PasteAsPlainText,
    #[doc(alias = "WEBKIT_CONTEXT_MENU_ACTION_CUSTOM")]
    Custom,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ContextMenuAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ContextMenuAction::{}",
            match *self {
                Self::NoAction => "NoAction",
                Self::OpenLink => "OpenLink",
                Self::OpenLinkInNewWindow => "OpenLinkInNewWindow",
                Self::DownloadLinkToDisk => "DownloadLinkToDisk",
                Self::CopyLinkToClipboard => "CopyLinkToClipboard",
                Self::OpenImageInNewWindow => "OpenImageInNewWindow",
                Self::DownloadImageToDisk => "DownloadImageToDisk",
                Self::CopyImageToClipboard => "CopyImageToClipboard",
                Self::CopyImageUrlToClipboard => "CopyImageUrlToClipboard",
                Self::OpenFrameInNewWindow => "OpenFrameInNewWindow",
                Self::GoBack => "GoBack",
                Self::GoForward => "GoForward",
                Self::Stop => "Stop",
                Self::Reload => "Reload",
                Self::Copy => "Copy",
                Self::Cut => "Cut",
                Self::Paste => "Paste",
                Self::Delete => "Delete",
                Self::SelectAll => "SelectAll",
                Self::InputMethods => "InputMethods",
                Self::Unicode => "Unicode",
                Self::SpellingGuess => "SpellingGuess",
                Self::NoGuessesFound => "NoGuessesFound",
                Self::IgnoreSpelling => "IgnoreSpelling",
                Self::LearnSpelling => "LearnSpelling",
                Self::IgnoreGrammar => "IgnoreGrammar",
                Self::FontMenu => "FontMenu",
                Self::Bold => "Bold",
                Self::Italic => "Italic",
                Self::Underline => "Underline",
                Self::Outline => "Outline",
                Self::InspectElement => "InspectElement",
                Self::OpenVideoInNewWindow => "OpenVideoInNewWindow",
                Self::OpenAudioInNewWindow => "OpenAudioInNewWindow",
                Self::CopyVideoLinkToClipboard => "CopyVideoLinkToClipboard",
                Self::CopyAudioLinkToClipboard => "CopyAudioLinkToClipboard",
                Self::ToggleMediaControls => "ToggleMediaControls",
                Self::ToggleMediaLoop => "ToggleMediaLoop",
                Self::EnterVideoFullscreen => "EnterVideoFullscreen",
                Self::MediaPlay => "MediaPlay",
                Self::MediaPause => "MediaPause",
                Self::MediaMute => "MediaMute",
                Self::DownloadVideoToDisk => "DownloadVideoToDisk",
                Self::DownloadAudioToDisk => "DownloadAudioToDisk",
                Self::InsertEmoji => "InsertEmoji",
                Self::PasteAsPlainText => "PasteAsPlainText",
                Self::Custom => "Custom",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for ContextMenuAction {
    type GlibType = ffi::WebKitContextMenuAction;

    fn into_glib(self) -> ffi::WebKitContextMenuAction {
        match self {
            Self::NoAction => ffi::WEBKIT_CONTEXT_MENU_ACTION_NO_ACTION,
            Self::OpenLink => ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK,
            Self::OpenLinkInNewWindow => ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK_IN_NEW_WINDOW,
            Self::DownloadLinkToDisk => ffi::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_LINK_TO_DISK,
            Self::CopyLinkToClipboard => ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_LINK_TO_CLIPBOARD,
            Self::OpenImageInNewWindow => ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_IMAGE_IN_NEW_WINDOW,
            Self::DownloadImageToDisk => ffi::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_IMAGE_TO_DISK,
            Self::CopyImageToClipboard => ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_TO_CLIPBOARD,
            Self::CopyImageUrlToClipboard => {
                ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_URL_TO_CLIPBOARD
            }
            Self::OpenFrameInNewWindow => ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_FRAME_IN_NEW_WINDOW,
            Self::GoBack => ffi::WEBKIT_CONTEXT_MENU_ACTION_GO_BACK,
            Self::GoForward => ffi::WEBKIT_CONTEXT_MENU_ACTION_GO_FORWARD,
            Self::Stop => ffi::WEBKIT_CONTEXT_MENU_ACTION_STOP,
            Self::Reload => ffi::WEBKIT_CONTEXT_MENU_ACTION_RELOAD,
            Self::Copy => ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY,
            Self::Cut => ffi::WEBKIT_CONTEXT_MENU_ACTION_CUT,
            Self::Paste => ffi::WEBKIT_CONTEXT_MENU_ACTION_PASTE,
            Self::Delete => ffi::WEBKIT_CONTEXT_MENU_ACTION_DELETE,
            Self::SelectAll => ffi::WEBKIT_CONTEXT_MENU_ACTION_SELECT_ALL,
            Self::InputMethods => ffi::WEBKIT_CONTEXT_MENU_ACTION_INPUT_METHODS,
            Self::Unicode => ffi::WEBKIT_CONTEXT_MENU_ACTION_UNICODE,
            Self::SpellingGuess => ffi::WEBKIT_CONTEXT_MENU_ACTION_SPELLING_GUESS,
            Self::NoGuessesFound => ffi::WEBKIT_CONTEXT_MENU_ACTION_NO_GUESSES_FOUND,
            Self::IgnoreSpelling => ffi::WEBKIT_CONTEXT_MENU_ACTION_IGNORE_SPELLING,
            Self::LearnSpelling => ffi::WEBKIT_CONTEXT_MENU_ACTION_LEARN_SPELLING,
            Self::IgnoreGrammar => ffi::WEBKIT_CONTEXT_MENU_ACTION_IGNORE_GRAMMAR,
            Self::FontMenu => ffi::WEBKIT_CONTEXT_MENU_ACTION_FONT_MENU,
            Self::Bold => ffi::WEBKIT_CONTEXT_MENU_ACTION_BOLD,
            Self::Italic => ffi::WEBKIT_CONTEXT_MENU_ACTION_ITALIC,
            Self::Underline => ffi::WEBKIT_CONTEXT_MENU_ACTION_UNDERLINE,
            Self::Outline => ffi::WEBKIT_CONTEXT_MENU_ACTION_OUTLINE,
            Self::InspectElement => ffi::WEBKIT_CONTEXT_MENU_ACTION_INSPECT_ELEMENT,
            Self::OpenVideoInNewWindow => ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_VIDEO_IN_NEW_WINDOW,
            Self::OpenAudioInNewWindow => ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_AUDIO_IN_NEW_WINDOW,
            Self::CopyVideoLinkToClipboard => {
                ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_VIDEO_LINK_TO_CLIPBOARD
            }
            Self::CopyAudioLinkToClipboard => {
                ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_AUDIO_LINK_TO_CLIPBOARD
            }
            Self::ToggleMediaControls => ffi::WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_CONTROLS,
            Self::ToggleMediaLoop => ffi::WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_LOOP,
            Self::EnterVideoFullscreen => ffi::WEBKIT_CONTEXT_MENU_ACTION_ENTER_VIDEO_FULLSCREEN,
            Self::MediaPlay => ffi::WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PLAY,
            Self::MediaPause => ffi::WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PAUSE,
            Self::MediaMute => ffi::WEBKIT_CONTEXT_MENU_ACTION_MEDIA_MUTE,
            Self::DownloadVideoToDisk => ffi::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_VIDEO_TO_DISK,
            Self::DownloadAudioToDisk => ffi::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_AUDIO_TO_DISK,
            Self::InsertEmoji => ffi::WEBKIT_CONTEXT_MENU_ACTION_INSERT_EMOJI,
            Self::PasteAsPlainText => ffi::WEBKIT_CONTEXT_MENU_ACTION_PASTE_AS_PLAIN_TEXT,
            Self::Custom => ffi::WEBKIT_CONTEXT_MENU_ACTION_CUSTOM,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WebKitContextMenuAction> for ContextMenuAction {
    unsafe fn from_glib(value: ffi::WebKitContextMenuAction) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::WEBKIT_CONTEXT_MENU_ACTION_NO_ACTION => Self::NoAction,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK => Self::OpenLink,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK_IN_NEW_WINDOW => Self::OpenLinkInNewWindow,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_LINK_TO_DISK => Self::DownloadLinkToDisk,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_LINK_TO_CLIPBOARD => Self::CopyLinkToClipboard,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_IMAGE_IN_NEW_WINDOW => Self::OpenImageInNewWindow,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_IMAGE_TO_DISK => Self::DownloadImageToDisk,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_TO_CLIPBOARD => Self::CopyImageToClipboard,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_URL_TO_CLIPBOARD => {
                Self::CopyImageUrlToClipboard
            }
            ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_FRAME_IN_NEW_WINDOW => Self::OpenFrameInNewWindow,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_GO_BACK => Self::GoBack,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_GO_FORWARD => Self::GoForward,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_STOP => Self::Stop,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_RELOAD => Self::Reload,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY => Self::Copy,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_CUT => Self::Cut,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_PASTE => Self::Paste,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_DELETE => Self::Delete,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_SELECT_ALL => Self::SelectAll,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_INPUT_METHODS => Self::InputMethods,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_UNICODE => Self::Unicode,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_SPELLING_GUESS => Self::SpellingGuess,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_NO_GUESSES_FOUND => Self::NoGuessesFound,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_IGNORE_SPELLING => Self::IgnoreSpelling,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_LEARN_SPELLING => Self::LearnSpelling,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_IGNORE_GRAMMAR => Self::IgnoreGrammar,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_FONT_MENU => Self::FontMenu,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_BOLD => Self::Bold,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_ITALIC => Self::Italic,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_UNDERLINE => Self::Underline,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_OUTLINE => Self::Outline,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_INSPECT_ELEMENT => Self::InspectElement,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_VIDEO_IN_NEW_WINDOW => Self::OpenVideoInNewWindow,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_AUDIO_IN_NEW_WINDOW => Self::OpenAudioInNewWindow,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_VIDEO_LINK_TO_CLIPBOARD => {
                Self::CopyVideoLinkToClipboard
            }
            ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_AUDIO_LINK_TO_CLIPBOARD => {
                Self::CopyAudioLinkToClipboard
            }
            ffi::WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_CONTROLS => Self::ToggleMediaControls,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_LOOP => Self::ToggleMediaLoop,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_ENTER_VIDEO_FULLSCREEN => Self::EnterVideoFullscreen,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PLAY => Self::MediaPlay,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PAUSE => Self::MediaPause,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_MEDIA_MUTE => Self::MediaMute,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_VIDEO_TO_DISK => Self::DownloadVideoToDisk,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_AUDIO_TO_DISK => Self::DownloadAudioToDisk,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_INSERT_EMOJI => Self::InsertEmoji,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_PASTE_AS_PLAIN_TEXT => Self::PasteAsPlainText,
            ffi::WEBKIT_CONTEXT_MENU_ACTION_CUSTOM => Self::Custom,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg_attr(feature = "v2_40", deprecated = "Since 2.40")]
#[cfg(any(feature = "v2_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WebKitFormSubmissionStep")]
pub enum FormSubmissionStep {
    #[doc(alias = "WEBKIT_FORM_SUBMISSION_WILL_SEND_DOM_EVENT")]
    SendDomEvent,
    #[doc(alias = "WEBKIT_FORM_SUBMISSION_WILL_COMPLETE")]
    Complete,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v2_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
impl fmt::Display for FormSubmissionStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FormSubmissionStep::{}",
            match *self {
                Self::SendDomEvent => "SendDomEvent",
                Self::Complete => "Complete",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "v2_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
#[doc(hidden)]
impl IntoGlib for FormSubmissionStep {
    type GlibType = ffi::WebKitFormSubmissionStep;

    fn into_glib(self) -> ffi::WebKitFormSubmissionStep {
        match self {
            Self::SendDomEvent => ffi::WEBKIT_FORM_SUBMISSION_WILL_SEND_DOM_EVENT,
            Self::Complete => ffi::WEBKIT_FORM_SUBMISSION_WILL_COMPLETE,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "v2_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
#[doc(hidden)]
impl FromGlib<ffi::WebKitFormSubmissionStep> for FormSubmissionStep {
    unsafe fn from_glib(value: ffi::WebKitFormSubmissionStep) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::WEBKIT_FORM_SUBMISSION_WILL_SEND_DOM_EVENT => Self::SendDomEvent,
            ffi::WEBKIT_FORM_SUBMISSION_WILL_COMPLETE => Self::Complete,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(any(feature = "v2_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
impl StaticType for FormSubmissionStep {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::webkit_form_submission_step_get_type()) }
    }
}

#[cfg(any(feature = "v2_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
impl glib::value::ValueType for FormSubmissionStep {
    type Type = Self;
}

#[cfg(any(feature = "v2_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
unsafe impl<'a> FromValue<'a> for FormSubmissionStep {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v2_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_20")))]
impl ToValue for FormSubmissionStep {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "WebKitUserMessageError")]
pub enum UserMessageError {
    #[doc(alias = "WEBKIT_USER_MESSAGE_UNHANDLED_MESSAGE")]
    UserMessageUnhandledMessage,
    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
impl fmt::Display for UserMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UserMessageError::{}",
            match *self {
                Self::UserMessageUnhandledMessage => "UserMessageUnhandledMessage",
                _ => "Unknown",
            }
        )
    }
}

#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
#[doc(hidden)]
impl IntoGlib for UserMessageError {
    type GlibType = ffi::WebKitUserMessageError;

    fn into_glib(self) -> ffi::WebKitUserMessageError {
        match self {
            Self::UserMessageUnhandledMessage => ffi::WEBKIT_USER_MESSAGE_UNHANDLED_MESSAGE,
            Self::__Unknown(value) => value,
        }
    }
}

#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
#[doc(hidden)]
impl FromGlib<ffi::WebKitUserMessageError> for UserMessageError {
    unsafe fn from_glib(value: ffi::WebKitUserMessageError) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::WEBKIT_USER_MESSAGE_UNHANDLED_MESSAGE => Self::UserMessageUnhandledMessage,
            value => Self::__Unknown(value),
        }
    }
}
