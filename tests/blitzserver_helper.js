const Helper = require('@codeceptjs/helper');

class Blitzserver extends Helper {

	proc

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
		if (this.proc) {
			this.proc.kill();
		}
	}

	// add custom methods here
	// If you need to access other helpers
	// use: this.helpers['helperName']


	async launchBlitzserver() {
		if(this.proc) {
			this.proc.kill();
		}
		const spawn = require('child_process').spawn;
		this.proc = spawn("target/debug/blitzserver", {cwd: ".."});
		async function sleep(ms) {
			return new Promise(resolve => setTimeout(resolve, ms));
		}
		await sleep(300);
	}
}

module.exports = Blitzserver;