const Helper = require('@codeceptjs/helper');

class Dom5 extends Helper {

	// before/after hooks
  /**
   * @protected
   */
	_before() {
		// remove if not used
	}

  /**
   * @protected
   */
	_after() {
		// remove if not used
	}

	// add custom methods here
	// If you need to access other helpers
	// use: this.helpers['helperName']
	async connectToServer(ip, port, name) {
		const util = require('util');
		const exec = util.promisify(require('child_process').exec);
		let {stdout} = await exec(this.config.binpath + " -C --tcpquery --ipadr " + ip + " --port " + port);
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
