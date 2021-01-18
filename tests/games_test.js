Feature('games');
const faker = require('faker');

Scenario('create game', async ({I}) => {
	let game_name = faker.name.firstName();
	I.amOnPage("/games");
	I.see("Create New Game");
	I.fillField("name", game_name);
	I.fillField("password", "password");
	I.click("Create");
	I.seeInCurrentUrl("settings");
	I.amOnPage("/game/" + game_name + "/status");
	let address = await I.grabTextFrom(".pane.status tr:first-child td:nth-child(2)");
	I.seeServerName(address.split(":")[0], address.split(":")[1], game_name);
});
