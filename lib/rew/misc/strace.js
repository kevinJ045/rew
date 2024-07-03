module.exports.straceLog = function(...logs){
  if(process.straceMode){
		console.log(`[${new Date().getHours()}:${new Date().getMinutes()}:${new Date().getSeconds()} ${new Date().getDate()}/${new Date().getMonth()}/${new Date().getFullYear()}] [${process.pid}]`, ...logs);
	}
}