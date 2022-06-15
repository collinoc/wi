use windows_sys::{
    Win32::Devices::DeviceAndDriverInstallation::*,
    Win32::Devices::Properties::DEVPKEY_Device_FriendlyName,
    Win32::Devices::Properties::DEVPROP_TYPE_STRING,
};

fn main() {
    unsafe {
        let instance = get_wifi_device();

        if instance == 0xFFFFFFFF {
            panic!("No wireless device found.");
        }

        let ret: CONFIGRET = CM_Disable_DevNode(instance, 0);

        if ret != 0 {
            panic!("Failed to disable device: error: {}", CM_MapCrToWin32Err(ret, 0));
        }

        println!("WiFi device disabled. Re-enabling in 5 seconds... ");

        std::thread::sleep(std::time::Duration::from_secs(5));
        
        let ret: CONFIGRET = CM_Enable_DevNode(instance, 0);
        
        if ret != 0 {
            panic!("Failed to re-enable device: error: {}", CM_MapCrToWin32Err(ret, 0));
        }
    }
}

unsafe fn get_wifi_device() -> u32 {
    let device_class_filter = b"{4d36e972-e325-11ce-bfc1-08002be10318}\0".as_ptr();

    let mut len = 0;

    let ret: CONFIGRET = CM_Get_Device_ID_List_SizeA(
        &mut len,
        device_class_filter,
        CM_GETIDLIST_FILTER_PRESENT | CM_GETIDLIST_FILTER_CLASS
    );

    if ret != 0 {
        panic!("Failed to get device list size: error: {}", CM_MapCrToWin32Err(ret, 0));
    }

    let mut buffer = vec![0u8; len as usize];

    let ret: CONFIGRET = CM_Get_Device_ID_ListA(
        device_class_filter,
        buffer.as_mut_ptr(),
        len,
        CM_GETIDLIST_FILTER_PRESENT | CM_GETIDLIST_FILTER_CLASS
    );

    if ret != 0 {
        panic!("Failed to get device list: error: {}", CM_MapCrToWin32Err(ret, 0));
    }

    let device_string = buffer.into_iter().map(|c| c as char).collect::<String>();

    let list = device_string.split('\0').filter(|s| s.len() > 1);

    for device in list {
        let mut friendly_name = [0u8; 256];
        let mut name_sz = 256;

        let id_addr = device.as_ptr() as *const i8;
        let mut instance_id = 0;
        
        let ret: CONFIGRET = CM_Locate_DevNodeA(
            &mut instance_id,
            id_addr,
            CM_LOCATE_DEVNODE_NOVALIDATION
        );

        if ret != 0 {
            panic!("Failed to get device instance id: error: {}", CM_MapCrToWin32Err(ret, 0));
        }
        
        let ret: CONFIGRET = CM_Get_DevNode_PropertyW(
            instance_id,
            &DEVPKEY_Device_FriendlyName,
            &mut DEVPROP_TYPE_STRING,
            friendly_name.as_mut_ptr(),
            &mut name_sz,
            0
        );

        if ret != 0 {
            panic!("Failed to get device property: error: {}", CM_MapCrToWin32Err(ret, 0));
        }

        let name = friendly_name.into_iter().map(|c| c as char).filter(|c| *c != '\0').collect::<String>();

        if name.to_ascii_lowercase().contains("wireless") {
            return instance_id;
        }
    }

    0xFFFFFFFF
}