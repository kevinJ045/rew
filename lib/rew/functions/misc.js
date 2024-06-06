
module.exports.scheduleFrame = function scheduleFrame(cb, immediate = false){
	return immediate ? setImmediate(cb) : setTimeout(cb, 1);
}

