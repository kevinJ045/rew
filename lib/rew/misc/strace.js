module.exports.straceLog = function(...logs){
  if(process.straceMode){
		console.log(`[${new Date().toLocaleTimeString()} ${new Date().getDate()}/${new Date().getMonth()}/${new Date().getFullYear()}] [${process.pid}]`, ...logs.map(i => i.replace(/^([=>]+)/, '\x1b[34m$1\x1b[39m').replace(/WARN:/g, '\x1b[30m\x1b[43m WARN \x1b[39m\x1b[49m').replace(/INFO/g, '\x1b[30m\x1b[44m INFO \x1b[39m\x1b[49m').replace(/^"(.+)"$/, '\x1b[32m$1\x1b[39m').replace(/!([A-Z-+]+)/, (_, a) => `\x1b[30m\x1b[4${a.startsWith('-') ? '5' : a.startsWith('+') ? '6' : '2'}m ${a.replace(/^[-+]/, '')} \x1b[39m\x1b[49m`)));
	}
}