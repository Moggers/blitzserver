const Helper = require('@codeceptjs/helper');

class Dom5 extends Helper {

	// before/after hooks
  /**
   * @protected
   */
	_before() {
	}

  /**
   * @protected
   */
	_after() {
	}

	async connectToServer(ip, port, name) {
		const util = require('util');
		const exec = util.promisify(require('child_process').exec);
		let {stdout} = await exec(this.config.binpath + " -C --tcpquery --ipadr " + ip + " --port " + port);
		console.log(stdout);
		let game_match = stdout.match(new RegExp("Gamename: ([^\n]+)"));
		if (game_match == null) {
			throw "Unable to connect to " + ip + ":" + port;
		}
		let game_name = game_match[1]
		if (game_name != name) {
			throw "Game name should be " + name + ", instead is " + game_name;
		}
	}
}

module.exports = Dom5;
