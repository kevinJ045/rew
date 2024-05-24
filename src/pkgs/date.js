// Date and Time Module

// Formatting Functions

/**
 * Format a date object to a string (YYYY-MM-DD).
 * @param {Date} date - The date object.
 * @return {string} - The formatted date string.
 */
function formatDate(date) {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

/**
 * Format a time object to a string (HH:MM:SS).
 * @param {Date} date - The date object.
 * @return {string} - The formatted time string.
 */
function formatTime(date) {
  const hours = String(date.getHours()).padStart(2, '0');
  const minutes = String(date.getMinutes()).padStart(2, '0');
  const seconds = String(date.getSeconds()).padStart(2, '0');
  return `${hours}:${minutes}:${seconds}`;
}

// Parsing Functions

/**
 * Parse a date string (YYYY-MM-DD) into a date object.
 * @param {string} dateString - The date string.
 * @return {Date} - The parsed date object.
 */
function parseDate(dateString) {
  const [year, month, day] = dateString.split('-').map(Number);
  return new Date(year, month - 1, day);
}

/**
 * Parse a time string (HH:MM:SS) into a date object.
 * @param {string} timeString - The time string.
 * @return {Date} - The parsed date object.
 */
function parseTime(timeString) {
  const [hours, minutes, seconds] = timeString.split(':').map(Number);
  const date = new Date();
  date.setHours(hours, minutes, seconds, 0);
  return date;
}

// Arithmetic Functions

/**
 * Add days to a date.
 * @param {Date} date - The date object.
 * @param {number} days - The number of days to add.
 * @return {Date} - The new date object.
 */
function addDays(date, days) {
  const result = new Date(date);
  result.setDate(result.getDate() + days);
  return result;
}

/**
 * Subtract days from a date.
 * @param {Date} date - The date object.
 * @param {number} days - The number of days to subtract.
 * @return {Date} - The new date object.
 */
function subtractDays(date, days) {
  return addDays(date, -days);
}

/**
 * Calculate the difference in days between two dates.
 * @param {Date} date1 - The first date object.
 * @param {Date} date2 - The second date object.
 * @return {number} - The difference in days.
 */
function differenceInDays(date1, date2) {
  const diffTime = date2 - date1;
  return Math.ceil(diffTime / (1000 * 60 * 60 * 24));
}


// Exporting the functions as a CommonJS module
module.exports = () => ({
  formatDate,
  formatTime,
  parseDate,
  parseTime,
  addDays,
  subtractDays,
  differenceInDays
});
