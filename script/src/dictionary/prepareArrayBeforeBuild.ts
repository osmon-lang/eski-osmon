//sort array before build
import * as fs from 'fs';
import { dictionary } from './dictionary';

/**
sorting array and saving it in json
 */
export const sortDictionaryToFile = (dicta: string[][]): void => {
    dicta.sort(function (a, b) {
        if (a[1].length < b[1].length) return 1;
        else if (a[1].length > b[1].length) return -1;
        else return 0;
    });

    const file = JSON.stringify(dicta, null, '\t').replace(/\\/g, '');

    fs.writeFile('src/dictionary/sortedOsmon.json', file, function (err) {
        if (err) return console.log(err);
        console.log('Osmonscript array sorted!');
    });
};

sortDictionaryToFile(dictionary);
