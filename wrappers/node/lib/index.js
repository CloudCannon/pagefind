import { PagefindService } from "./service.js";

/**
 * @typedef {import('pagefindInternal').InternalResponseCallback} InternalResponseCallback
 * @typedef {import('pagefindInternal').InternalResponsePayload} InternalResponsePayload
 */

/**
 * @type {PagefindService?}
 */
let persistentService;
const launch = () => {
    if (!persistentService) {
        persistentService = new PagefindService();
    }
    return persistentService;
}

/**
 * @template T
 * @param {function(any): void} resolve 
 * @param {function(any): void} reject 
 * @param {InternalResponseCallback} response_callback 
 * @param {function(InternalResponsePayload): T} resultFn 
 */
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

/** 
 * @typedef {import('pagefindService').NewIndexResponse} NewIndexResponse
 * 
 * @type {import('pagefindService').createIndex} 
 * */
export const createIndex = () => new Promise((resolve, reject) => {
    const action = 'NewIndex';
    launch().sendMessage(
        {
            type: action
        }, (response) => {
            /** @type {function(InternalResponsePayload): Omit<NewIndexResponse, 'errors'>?} */
            const successCallback = (success) => {
                if (success.type !== action) {
                    reject(`Message returned from backend should have been ${action}, but was ${success.type}`);
                    return null;
                }

                return {
                    index: indexFns(success.index_id),
                }
            };
            handleApiResponse(resolve, reject, response, successCallback);
        }
    );
});

/**
 * @param {number} indexId 
 * @returns {import ('pagefindService').PagefindIndex}
 */
const indexFns = (indexId) => {
    return {
        addHTMLFile: (file) => addHTMLFile(indexId, file),
        addCustomRecord: (record) => addCustomRecord(indexId, record),
        writeFiles: () => writeFiles(indexId)
    }
}

/**
 * @typedef {import('pagefindService').NewFileResponse} NewFileResponse
 * 
 * @param {number} indexId 
 * @param {import('pagefindService').HTMLFile} file
 * @returns {Promise<NewFileResponse>}
 */
const addHTMLFile = (indexId, file) => new Promise((resolve, reject) => {
    const action = 'AddFile';
    const responseAction = 'IndexedFile';
    launch().sendMessage(
        {
            type: action,
            index_id: indexId,
            file_path: file.path,
            file_contents: file.content
        }, (response) => {
            /** @type {function(InternalResponsePayload): Omit<NewFileResponse, 'errors'>?} */
            const successCallback = (success) => {
                if (success.type !== responseAction) {
                    reject(`Message returned from backend should have been ${action}, but was ${success.type}`);
                    return null;
                }

                return {
                    file: {
                        uniqueWords: success.page_word_count,
                        url: success.page_url,
                        meta: success.page_meta,
                    }
                }
            };
            handleApiResponse(resolve, reject, response, successCallback);
        }
    );
});

/**
 * @param {number} indexId 
 * @param {import('pagefindService').CustomRecord} record
 * @returns {Promise<NewFileResponse>}
 */
const addCustomRecord = (indexId, record) => new Promise((resolve, reject) => {
    const action = 'AddRecord';
    const responseAction = 'IndexedFile';
    launch().sendMessage(
        {
            type: action,
            index_id: indexId,
            url: record.url,
            content: record.content,
            language: record.language,
            meta: record.meta,
            filters: record.filters,
            sort: record.sort,
        }, (response) => {
            /** @type {function(InternalResponsePayload): Omit<NewFileResponse, 'errors'>?} */
            const successCallback = (success) => {
                if (success.type !== responseAction) {
                    reject(`Message returned from backend should have been ${action}, but was ${success.type}`);
                    return null;
                }

                return {
                    file: {
                        uniqueWords: success.page_word_count,
                        url: success.page_url,
                        meta: success.page_meta,
                    }
                }
            };
            handleApiResponse(resolve, reject, response, successCallback);
        }
    );
});

/**
 * @typedef {import ('pagefindService').WriteFilesResponse} WriteFilesResponse
 * 
 * @param {number} indexId 
 * @returns {Promise<WriteFilesResponse>}
 */
const writeFiles = (indexId) => new Promise((resolve, reject) => {
    const action = 'WriteFiles';
    launch().sendMessage(
        {
            type: action,
            index_id: indexId,
        }, (response) => {
            /** @type {function(InternalResponsePayload): Omit<WriteFilesResponse, 'errors'>?} */
            const successCallback = (success) => {
                if (success.type !== action) {
                    reject(`Message returned from backend should have been ${action}, but was ${success.type}`);
                    return null;
                }

                return {
                    bundleLocation: success.bundle_location
                }
            };
            handleApiResponse(resolve, reject, response, successCallback);
        }
    );
});