import { load } from '@tauri-apps/plugin-store';

export const getPageData = async (key) => {
    const pageInitStore = await load('page.json.', {
        autoSave: true,
    });
    try {
        let oldValue = await pageInitStore.get(key);
        return oldValue;
    } catch (error) {
        return ''
    }
}

export const updatePageData = async (key, value) => {
    const pageInitStore = await load('page.json.', {
        autoSave: true,
    });
    console.log('updatePageData', key, value)
    await pageInitStore.set(key, value);
}


export const getPageDataJson = async (key) => {
    try {
        let value = await getPageData(key);
        let jsonData = JSON.parse(value);
        return jsonData;
    } catch (error) {
        return null;
    }   
}

export const updatePageDataJson = async (key, value) => {
    let jsonData = JSON.stringify(value);
    await updatePageData(key, jsonData);
}