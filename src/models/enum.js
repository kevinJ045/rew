

module.exports.cenum = function cenum(values) {
  var enumObj, i, len, value;
  // Create an object to hold the enum values
  enumObj = {};
  for (i = 0, len = values.length; i < len; i++) {
    value = values[i];
    enumObj[value] = value;
  }
  // Add a method to check if a value is a valid enum value
  enumObj.isValid = function(val) {
    return indexOf.call(enumObj, val) >= 0;
  };
  return enumObj;
};
