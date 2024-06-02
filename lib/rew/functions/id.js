module.exports.generateRandomID = function generateRandomID(length = 12) {
	const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
	const charactersLength = characters.length;
	let randomID = '';

	for (let i = 0; i < length; i++) {
		const randomIndex = Math.floor(Math.random() * charactersLength);
		randomID += characters.charAt(randomIndex);
	}

	return randomID;
};
