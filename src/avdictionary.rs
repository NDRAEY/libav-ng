use std::ffi::{CStr, CString};

use libav_sys_ng::{
    self, av_dict_copy, av_dict_count, av_dict_free, av_dict_get, av_dict_set, AVDictionary, AVDictionaryEntry, AV_DICT_IGNORE_SUFFIX
};

pub struct Dictionary {
    _dict: *mut AVDictionary,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Self {
            _dict: core::ptr::null_mut(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let k = CString::new(key).expect("Failed to create key for AVDictionary");
        let v = CString::new(value).expect("Failed to create value for AVDictionary");

        unsafe {
            av_dict_set(&mut self._dict, k.as_ptr(), v.as_ptr(), 0);
        }
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        let k = CString::new(key).expect("Failed to create key for AVDictionary");

        unsafe {
            let value = av_dict_get(self._dict, k.as_ptr(), core::ptr::null(), 0);

            if value.is_null() {
                return None;
            }

            let raw_value = (*value).value;

            let mw = CStr::from_ptr(raw_value)
                .to_str()
                .expect("Failed to convert!")
                .to_string(); 

            return Some(mw);
        }
    }

    pub fn count(&self) -> i32 {
        unsafe { av_dict_count(self._dict) }
    }

    pub(crate) unsafe fn raw(&mut self) -> *mut AVDictionary {
        self._dict
    }
}

#[derive(Debug, Clone)]
pub struct DictionaryEntry {
    key: String,
    value: String,
}

impl DictionaryEntry {
    pub fn from_raw(entry: &AVDictionaryEntry) -> DictionaryEntry {
        unsafe {
            let k = CStr::from_ptr(entry.key)
                .to_str()
                .expect("Failed to convert!")
                .to_string();
            let v = CStr::from_ptr(entry.value)
                .to_str()
                .expect("Failed to convert!")
                .to_string();

            DictionaryEntry { key: k, value: v }
        }
    }
}

impl Clone for Dictionary {
    fn clone(&self) -> Dictionary {
        let mut new_av = core::ptr::null_mut::<AVDictionary>();

        let result = unsafe { av_dict_copy(&mut new_av, self._dict, AV_DICT_IGNORE_SUFFIX as i32) };

        if result != 0 {
            // TODO: What I should do with this? Panic? Do not implement clone and write own clone in
            // Dictionary impl?
            todo!("Failed to clone Dictionary");
        }

        Dictionary { _dict: new_av }
    }
}

pub struct DictionaryIter {
    dict: Dictionary,
    prev: *mut AVDictionaryEntry,
}

/// I don't know how to make human-friendly `for i in &dict`
impl Iterator for DictionaryIter {
    type Item = DictionaryEntry;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let k = CString::new("").expect("Failed to create key for AVDictionary");
            self.prev = av_dict_get(
                self.dict.raw(),
                k.as_ptr(),
                self.prev,
                AV_DICT_IGNORE_SUFFIX as i32,
            );

            if self.prev.is_null() {
                return None;
            }

            Some(DictionaryEntry::from_raw(&*self.prev))
        }
    }
}

impl IntoIterator for Dictionary {
    type Item = DictionaryEntry;
    type IntoIter = DictionaryIter;

    fn into_iter(self) -> Self::IntoIter {
        DictionaryIter {
            dict: self,
            prev: core::ptr::null_mut(),
        }
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe {
            av_dict_free(&mut self._dict);
        }
    }
}

/// Tests
#[cfg(test)]
mod tests {
    use super::Dictionary;

    #[test]
    fn simple() {
        let mut dict = Dictionary::new();

        dict.set("hello", "world");
        dict.set("pokemon", "zeraora");
        dict.set("thanks", "kiitos");
        dict.set("you're welcome!", "ole hyvä!");

        // The output will be silenced, but it tests SIGSEGV and SIGABRT
        for i in dict.clone() {
            println!("Element: {:?}", i);
        }

        assert_eq!(dict.get("pokemon"), Some("zeraora".to_string()));

        println!("Finished");
    }

    #[test]
    fn oops_undefined() {
        let mut dict = Dictionary::new();

        dict.set("hello", "world");
 
        assert_eq!(dict.get("hallo"), None);
        assert_eq!(dict.get("hello"), Some("world".to_string()));
    }
}
