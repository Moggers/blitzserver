<?php
namespace App\Test\TestCase\Model\Table;

use App\Model\Table\MatchnationturnsTable;
use Cake\ORM\TableRegistry;
use Cake\TestSuite\TestCase;

/**
 * App\Model\Table\MatchnationturnsTable Test Case
 */
class MatchnationturnsTableTest extends TestCase
{

    /**
     * Test subject
     *
     * @var \App\Model\Table\MatchnationturnsTable
     */
    public $Matchnationturns;

    /**
     * Fixtures
     *
     * @var array
     */
    public $fixtures = [
        'app.matchnationturns',
        'app.matchnations',
        'app.nations',
        'app.matches',
        'app.turns',
        'app.maps',
        'app.mods',
        'app.matchmods',
        'app.emailrequests'
    ];

    /**
     * setUp method
     *
     * @return void
     */
    public function setUp()
    {
        parent::setUp();
        $config = TableRegistry::exists('Matchnationturns') ? [] : ['className' => 'App\Model\Table\MatchnationturnsTable'];
        $this->Matchnationturns = TableRegistry::get('Matchnationturns', $config);
    }

    /**
     * tearDown method
     *
     * @return void
     */
    public function tearDown()
    {
        unset($this->Matchnationturns);

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
