const { typeis } = require("../const/default");
const RuneDB = require("./modules/rune/db");



module.exports = (context) => ({
  schema(runeDB) {
    if (!runeDB) throw new ReferenceError('You should pass a rune database for the schema.');
    if (!(runeDB instanceof RuneDB)) throw new TypeError('First argument is not a Rune database.');

    const models = {};

    const validateFields = (definition, data) => {
      const validatedData = {};
      for (const [field, type] of Object.entries(definition)) {
        if (data[field] === undefined) continue;
        const value = data[field];
        if (!typeis(value, type) && value != type) {
          throw new TypeError(`Invalid type for field "${field}". Expected ${
            type?.type?.type || type?.type || type
          }, got ${typeof value}`);
        }
        validatedData[field] = value;
      }
      return validatedData;
    };

    const applyFieldSelection = (record, select) => {
      if (!select) return record;
      if (Array.isArray(select)) select = Object.fromEntries(select.map(i => [i, true]));
      const selectedRecord = {};
      for (const key of Object.keys(select)) {
        if (select[key]) selectedRecord[key] = record[key];
      }
      return selectedRecord;
    };

    return {
      model(modelName, definition, options = {}) {
        if (models[modelName]) throw new Error(`Model "${modelName}" is already defined.`);

        const collection = runeDB.collection(modelName);

        const modelAPI = {
          create(record) {
            const validatedRecord = validateFields(definition, record);
            return collection.insert(validatedRecord);
          },

          findUnique(id, select) {
            const record = collection.read(id);
            if (!record) return null;

            let finalRecord = record;
            return applyFieldSelection(finalRecord, select);
          },

          findMany(where, select, cursor, count = 0) {
            
            const records = collection.find(where, count == 0 ? -1 : count, cursor);

            return select
              ? records.map((record) => applyFieldSelection(record, select))
              : records;
          },

          update(identifier, updates, limit) {
            return collection.update(identifier, updates, limit);
          },

          delete(identifier, limit) {
            return collection.remove(identifier, limit);
          },

          list() {
            return collection.list();
          },

          empty(){
            return collection.empty();
          }
        };

        models[modelName] = modelAPI;
        return modelAPI;
      },

      useModel(modelName) {
        if (!models[modelName]) throw new Error(`Model "${modelName}" is not defined.`);
        return models[modelName];
      },
    };
  },
});
