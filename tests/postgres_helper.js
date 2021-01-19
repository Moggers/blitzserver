const Helper = require('@codeceptjs/helper');
const {Client} = require('pg');

class Postgres extends Helper {

	pgClient

	// before/after hooks
  /**
   * @protected
   */
	async _before() {
		this.pgClient = new Client(this.config.connectionString);
		await this.pgClient.connect();
	}

  /**
   * @protected
   */
	async _after() {
		await this.pgClient.end();
	}

	async clearDatabase() {
		await this.pgClient.query(`
			TRUNCATE games CASCADE;
			TRUNCATE maps CASCADE;
			TRUNCATE email_configs CASCADE;
			TRUNCATE players CASCADE;
			TRUNCATE nations CASCADE;
			TRUNCATE mods CASCADE;
			`);
	}
}

module.exports = Postgres;
