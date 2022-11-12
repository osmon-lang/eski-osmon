/*
initially array is unsorted
 */
export const dictionary = [
    ['break', 'toxta'],
    ['case', 'holat'],
    ['catch', 'tut'],
    ['continue', 'tashla'],
    ['debugger', 'debagger'],
    ['delete', 'ochir'],
    ['do', 'bajar'],
    ['else', 'aks'],
    ['finally', 'oxiri'],
    ['for', 'har'],
    ['function', 'funksiya'],
    ['function*', 'funksiya*'],
    ['if', 'agar'],
    ['in', 'ichida'],
    ['default', 'odatiy'],
    ['instanceof', 'obyektTuri'],
    ['new', 'yangi'],
    ['return', 'qaytar'],
    ['yield', 'ber'],
    ['yield*', 'ber*'],
    ['switch', 'tanlov'],
    ['this', 'shu'],
    ['throw', 'otvor'],
    ['try', 'urin'],
    ['typeof', 'turi'],
    ['var', 'ozg'],
    ['let', 'joy'],
    ['void', 'bosh'],
    ['while', 'qachonki'],
    ['with', 'bilan'],
    ['Abstract', 'Abstrakt'],
    ['abstract', 'abstrakt'],
    ['Boolean', 'Mantiq'],
    ['boolean', 'mantiq'],
    ['Byte', 'Bayt'],
    ['byte', 'bayt'],
    ['Char', 'Ramz'],
    ['char', 'ramz'],
    ['class', 'klass'],
    ['Const', 'Doimiy'],
    ['const', 'doimiy'],
    ['Double', 'Ikkitalik'],
    ['double', 'ikkitalik'],
    ['Enum', 'Tur'],
    ['enum', 'tur'],
    ['extends', 'uzaytiradi'],
    ['final', 'songgi'],
    ['Float', 'Kasr'],
    ['float', 'kasr'],
    ['goto', 'bor'],
    ['implements', 'amallar'],
    ['import', 'ishlat'],
    ['Int', 'Butun'],
    ['int', 'butun'],
    ['interface', 'tuzilish'],
    ['Long', 'Uzun'],
    ['long', 'uzun'],
    ['native', 'native'],
    ['package', 'paket'],
    ['private', 'shaxsiy'],
    ['protected', 'himoyalangan'],
    ['public', 'ommaviy'],
    ['Short', 'Kalta'],
    ['short', 'kalta'],
    ['static', 'statik'],
    ['super', 'super'],
    ['synchronized', 'sinxron'],
    ['throws', 'tashla'],
    ['transient', 'ахз'],
    ['volatile', 'volatile'],
    ['null', 'nol'],
    ['NaN', 'RE'],
    ['undefined', 'aniqlanmagan'],
    ['true', 'ha'],
    ['false', 'yoq'],
    ['eval', 'qil'],
    // ['\'use strict\'', '\'qatiyFoydalan\''], Bag: global.osmon("'use strict'", 'js') qaytaradi 'use strict'
    //    operators
    ['\\{', '{'],
    ['\\}', '}'],
    ['\\=\\=', '=='],
    ['\\=\\=\\=', '==='],
    ['\\>\\=', '>='],
    ['\\<\\=', '<='],
    ['\\&\\&', 'va'],
    ['\\|\\|', 'yoki'],
    ['\\>', '>'],
    ['\\<', '<'],
    ['\\=', '='],
    ['\\;', ' ;'],
    ['\\!', '!'],
    ['\\+\\+', '++'],
    ['\\-\\-', '--'],
    //    Document methods
    ['document', 'hujjat'],
    ['captureEvents', 'hodisalarniOl'],
    ['createAttribute', 'attributYarat'],
    ['createDocumentFragment', 'hujjatQismYarat'],
    ['createEvent', 'hodisaYarat'],
    ['createNodeIterator', 'tugunIteratorYarat'],
    ['createRange', 'chegaraYarat'],
    ['createTextNode', 'tugunTekstYarat'],
    ['createElement', 'elementYarat'],
    ['getElementsByClassName', 'klassdanElementOl'],
    ['getElementById', 'idDanElementOl'],
    ['importNode', 'ishlatTugun'],
    ['clear', 'tozala'],
    ['close', 'yop'],
    ['execCommand', 'buyruqBajarish'],
    ['write', 'yoz'],
    ['writeln', 'yozqt'],
    //    Document Properties

    ['URL', 'havola'],

    ['value', 'qiymat'],
    //    Document event handlers

    //    Global event handlers

    //    Window properties
    ['window', 'oyna'],
    //    Window methods
    ['addEventListener', 'hodisaKuzatuvchiQosh'],
    ['alert', 'yozuv'],
    ['find', 'top'],
    ['focus', 'fokus'],
    ['getAttention', 'etiborQilish'],
    ['maximize', 'maksimallashtir'],
    ['minimize', 'minimallashtir'],
    ['moveBy', 'obor'],
    ['moveTo', 'oborish'],
    ['openDialog', 'dialogOch'],
    ['print', 'chiqarish'],

    //    Window event handlers

    //    Node properties

    //    Node methods

    //    String properties

    //    String methods

    ['search', 'izla'],
    ['slice', 'kes'],
    ['split', 'bol'],

    //    String HTML wrapper methods

    //    Canvas properties

    //    Canvas methods

    //    Number properties

    //    Number methods

    //    Console methods

    ['count', 'sana'],
    ['error', 'xatolik'],
    ['group', 'guruh'],
    ['groupEnd', 'guruhTugash'],
    ['info', 'malumot'],
    ['log', 'log'],
    ['profile', 'profil'],
    ['profileEnd', 'profilTugash'],
    ['table', 'jadval'],
    ['time', 'vaqt'],
    ['timeEnd', 'vaqtTugash'],

    //    XMLHttpRequest properties

    //    XMLHttpRequest methods

    //    XMLHttpRequest Inheritance

    //    XMLHttpRequest events

    //    Arrays properties
    ['Array', 'massiv'],
    //    Arrays methods
    ['from', 'dan'],
    ['isArray', 'massivMi'],
    ['map', 'toplam'],
    ['of', 'dan'],
    ['of', 'of'],
    ['values', 'qiymatlar'],
    //    Math properties
    ['Math', 'Matem'],
    ['Math', 'Matematika'],

    //math methods
    //    RegExp properties
    // ['input', 'yozuvchi'],
    // ['flags', 'bayroqlar'],
    // ['global', 'globalReg'],
    // ['ignoreCase', 'ignor'],
    // ['multiline', 'kopqatorlik'],
    // ['source', 'manbaa'],

    //    async/await functions

    //    Promise methods
    ['all', 'hamma'],
    ['then', 'song'],
    //    Object properties
    ['Object', 'Obyekt'],
    ['constructor', 'yarat'],
    //    Object methods
    ['assign', 'tayinla'],

    //    NodeJS/modules support
    ['module', 'qoshimcha'],
    ['exports', 'eksportlar'],
    ['export', 'eksport'],
    ['global', 'global'],
];