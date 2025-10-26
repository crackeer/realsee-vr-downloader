import { invoke as tauriInvoke } from "@tauri-apps/api/core";

const invoke = async (...args) => {
    try {
        return await tauriInvoke(...args)
    } catch (e) {
        console.log('invoke exception', e, ...args)
    }
}

export async function writeFile(file, content) {
    let result = await invoke("write_file", {
        filePath: file,
        content: content,
    });
    return result;

}

export async function readFile (file) {
    let result = await invoke("read_file", {
        filePath: file,
    });
    return result;
};


export async function fileExists(filePath) {
    let result = await invoke("file_exists", {
        filePath: filePath,
    });
    return result;
};

export async function deleteFile(filePath) {
    let result = await invoke("file_exists", {
        filePath: filePath,
    });
    return result;
}


 export  async function downloadFile(url, dest) {
    console.log("downloadFile", url, dest);
    return await invoke("download_file", {
        url: url,
        savePath: dest,
    });
};


export async function downloadFileWithContent(url, dest) {
    console.log("downloadFileWithContent", url, dest);
    return await invoke("download_file_with_content", {
        url: url,
        savePath: dest,
    });
};


export async function createJSONPFile  (src, dest, hash) {
    return await invoke("create_jsonp_file", {
        srcFile: src,
        destFile: dest,
        hashCode: hash,
    });
};

export async function writeRsvrJsonpAsset(dir) {
    return await invoke("write_rsvr_jsonp_asset", {
        dir,
    });
};

export async function openFile(filePath) {
    return await invoke("open_file", {
        filePath,
    });
};

export async function isDev() {
    return true
}

export async function openPath(path) {
     return await invoke("open_path", {
        filePath : path,
    });
}