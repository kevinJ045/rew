const { pickRandom } = require("../const/default")

const alphabets = 'abcdefghijklmnopqrstuvwxyz';
const numericals = '0123456789';

module.exports = () => ({
  alphabets,
  numericals,
  rn(length = 1){
    return Array(length || 1).fill(0).map(() => pickRandom(...numericals.split(''))).join('');
  },
  rl(length = 1){
    return Array(length || 1).fill(0).map(() => pickRandom(...alphabets.split(''))).join('');
  },
  rnl(length = 1){
    return Array(length || 1).fill(0).map(() => pickRandom(...(alphabets + numericals).split(''))).join('');
  }
})