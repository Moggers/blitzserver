<?php
namespace App\Test\TestCase\Model\Table;

use App\Model\Table\EmailrequestsTable;
use Cake\ORM\TableRegistry;
use Cake\TestSuite\TestCase;

/**
 * App\Model\Table\EmailrequestsTable Test Case
 */
class EmailrequestsTableTest extends TestCase
{

    /**
     * Test subject
     *
     * @var \App\Model\Table\EmailrequestsTable
     */
    public $Emailrequests;

    /**
     * Fixtures
     *
     * @var array
     */
    public $fixtures = [
        'app.emailrequests',
        'app.emailjobs'
    ];

    /**
     * setUp method
     *
     * @return void
     */
    public function setUp()
    {
        parent::setUp();
        $config = TableRegistry::exists('Emailrequests') ? [] : ['className' => 'App\Model\Table\EmailrequestsTable'];
        $this->Emailrequests = TableRegistry::get('Emailrequests', $config);
    }

    /**
     * tearDown method
     *
     * @return void
     */
    public function tearDown()
    {
        unset($this->Emailrequests);

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
}
