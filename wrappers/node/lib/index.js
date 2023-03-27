import { PagefindService } from "./service.js";

let persistentService;
const launch = () => {
    if (!persistentService) {
        persistentService = new PagefindService();
    }
    return persistentService;
}

const handleApiResponse = (resolve, reject, { exception, err, result }, resultFn) => {
    if (exception) {
        reject(exception);
    } else {
        resolve({
            errors: err ? [err.message] : [],
            ...(result ? resultFn(result) : {})
        });
    }
}

export const createIndex = () => new Promise((resolve, reject) => {
    launch().sendMessage(
        {
            type: 'NewIndex'
        }, (response) => {
            handleApiResponse(resolve, reject, response, (success) => {
                return {
                    index: indexFns(success.index_id)
                }
            });
        }
    );
});

const indexFns = (indexId) => {
    return {
        addFile: (filePath, fileContents) => addFile(indexId, filePath, fileContents),
        writeFiles: () => writeFiles(indexId)
    }
}

const addFile = (indexId, filePath, fileContents) => new Promise((resolve, reject) => {
    launch().sendMessage(
        {
            type: 'AddFile',
            index_id: indexId,
            file_path: filePath,
            file_contents: fileContents
        }, (response) => {
            handleApiResponse(resolve, reject, response, (success) => {
                return {
                    uniqueWords: success.page_word_count,
                    url: success.page_url,
                    meta: success.page_meta,
                }
            });
        }
    );
});

const writeFiles = (indexId) => new Promise((resolve, reject) => {
    launch().sendMessage(
        {
            type: 'WriteFiles'
        }, (err, res) => err ? reject(err) : resolve(res)
    );
});