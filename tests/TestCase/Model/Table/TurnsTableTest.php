<?php
namespace App\Test\TestCase\Model\Table;

use App\Model\Table\TurnsTable;
use Cake\ORM\TableRegistry;
use Cake\TestSuite\TestCase;

/**
 * App\Model\Table\TurnsTable Test Case
 */
class TurnsTableTest extends TestCase
{

    /**
     * Test subject
     *
     * @var \App\Model\Table\TurnsTable
     */
    public $Turns;

    /**
     * Fixtures
     *
     * @var array
     */
    public $fixtures = [
        'app.turns',
        'app.matches',
        'app.maps',
        'app.nations',
        'app.matchnations',
        'app.mods',
        'app.matchmods'
    ];

    /**
     * setUp method
     *
     * @return void
     */
    public function setUp()
    {
        parent::setUp();
        $config = TableRegistry::exists('Turns') ? [] : ['className' => 'App\Model\Table\TurnsTable'];
        $this->Turns = TableRegistry::get('Turns', $config);
    }

    /**
     * tearDown method
     *
     * @return void
     */
    public function tearDown()
    {
        unset($this->Turns);

        parent::tearDown();
    }

    /**
     * Test initialize method
     *
     * @return void
     */
    public function testInitialize()
    {
        $this->markTestIncomplete('Not implemented yet.');
    }

    /**
     * Test validationDefault method
     *
     * @return void
     */
    public function testValidationDefault()
    {
        $this->markTestIncomplete('Not implemented yet.');
    }

    /**
     * Test buildRules method
     *
     * @return void
     */
    public function testBuildRules()
    {
        $this->markTestIncomplete('Not implemented yet.');
    }
}
