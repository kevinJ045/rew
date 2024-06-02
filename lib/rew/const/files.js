const fs = require('fs');
const path = require('path');

const HOME_PATH = path.resolve(process.env.HOME, '.config/rew/default');
const THEME_PATH = (module.exports.THEME_PATH = path.resolve(HOME_PATH, 'theme.css'));

module.exports.FILES = [
	{
		path: HOME_PATH,
	},
	{
		path: THEME_PATH,
		content: fs.readFileSync(path.resolve(__dirname, '../css/theme.css')),
	},
];
