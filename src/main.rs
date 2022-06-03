use windows_sys::{
    Win32::Devices::DeviceAndDriverInstallation::*,
};

fn main() {
    unsafe {
        let instance = get_wifi_device();

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

    let mut len: u32 = 0;

    let ret: CONFIGRET = CM_Get_Device_ID_List_SizeA(
        &mut len,
        device_class_filter,
        CM_GETIDLIST_FILTER_PRESENT | CM_GETIDLIST_FILTER_CLASS
    );

    if ret != 0 {
        panic!("Failed to get device list size: error: {}", CM_MapCrToWin32Err(ret, 0));
    }

    let mut buffer = vec![0 as u8; len as usize];

    let ret: CONFIGRET = CM_Get_Device_ID_ListA(
        device_class_filter,
        buffer.as_mut_ptr(),
        len,
        CM_GETIDLIST_FILTER_PRESENT | CM_GETIDLIST_FILTER_CLASS
    );

    if ret != 0 {
        panic!("Failed to get device list: error: {}", CM_MapCrToWin32Err(ret, 0));
    }

    let list = buffer.into_iter().map(|c| c as char).collect::<String>();

    let wifi_device = list.split("\\0").nth(1).unwrap().to_string();

    let id_string = wifi_device.split('\0').nth(2).unwrap();

    let id_addr = id_string.as_ptr() as *const i8;

    let mut instance_id: u32 = 0;
    
    let ret: CONFIGRET = CM_Locate_DevNodeA(
        &mut instance_id,
        id_addr,
        CM_LOCATE_DEVNODE_NOVALIDATION
    );

    if ret != 0 {
        panic!("Failed to get device instance id: error: {}", CM_MapCrToWin32Err(ret, 0));
    }

    instance_id
}