"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var RNSqlite = require('react-native-sqlcipher-storage')
class Database {
    constructor(filename, callback) {
        this._db = RNSqlite.openDatabase({
            name: filename,
            location: 'docs',
            key: 'test_key'
        }, null, (error) => callback && callback(error));
    }
    close(callback) {
        if (this._db) {
            this._db.close(null, callback);
            this._db = null;
        }
        else {
            callback.call(null, new Error('No open database'));
        }
    }
    run(sql, ...params) {
        const callback = params.pop();
        this._db.executeSql(sql, params, (resultSet) => {
            callback.call(mkResultCtx(this), null);
        }, (err) => {
            callback.call(null, new Error(err.message));
        });
        return this;
    }
    ;
    all(sql, ...params) {
        const callback = params.pop();
        this._db.executeSql(sql, params, (resultSet) => {
            callback.call(mkResultCtx(resultSet), null, resultSet.rows.raw());
        }, (err) => {
            callback.call(null, new Error(err.message));
        });
        return this;
    }
    ;
    each(sql, ...params) {
        const completed = params.pop();
        const callback = params.pop();
        this._db.executeSql(sql, params, (resultSet) => {
            for (let i = 0; i < resultSet.rows.length; i++) {
                callback.call(null, null, resultSet.rows.item(i));
            }
            complete.call(null, null, resultSet.rows.length);
        }, (err) => {
            callback.call(null, new Error(err.message));
            complete.call(null, new Error(err.message));
        });
        return this;
    }
    ;
    exec(sql, callback) {
        this._db.executeSql(sql, [], () => {
            callback.call(null);
        }, (err) => {
            callback.call(null, new Error(err.message));
        });
        return this;
    }
    ;
}
exports.Database = Database;
function mkResultCtx(resultSet) {
    return {
        lastID: resultSet.insertId,
        changes: resultSet.rowsAffected
    };
}
//# sourceMappingURL=RNSqlite3Adapter.js.map
