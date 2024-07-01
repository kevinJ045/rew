Function.prototype.wait = function(...args) {
	return wait(this, ...args);
};
Function.prototype.globe = () => global