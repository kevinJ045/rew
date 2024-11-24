Function.prototype.wait = function(...args) {
	return wait(this, ...args);
};
Object.without = function(object, ...keys){
	let newObject = {...object};
	for(let i = 0; i < keys.length; i++){
		delete newObject[keys[i]];
	}
	return newObject;
}