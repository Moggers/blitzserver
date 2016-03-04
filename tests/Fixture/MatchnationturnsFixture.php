<?php
namespace App\Test\Fixture;

use Cake\TestSuite\Fixture\TestFixture;

/**
 * MatchnationturnsFixture
 *
 */
class MatchnationturnsFixture extends TestFixture
{

    /**
     * Fields
     *
     * @var array
     */
    // @codingStandardsIgnoreStart
    public $fields = [
        'id' => ['type' => 'integer', 'length' => 11, 'unsigned' => false, 'null' => false, 'default' => null, 'comment' => '', 'autoIncrement' => true, 'precision' => null],
        'matchnation_id' => ['type' => 'integer', 'length' => 11, 'unsigned' => false, 'null' => false, 'default' => null, 'comment' => '', 'precision' => null, 'autoIncrement' => null],
        'turn_id' => ['type' => 'integer', 'length' => 11, 'unsigned' => false, 'null' => false, 'default' => null, 'comment' => '', 'precision' => null, 'autoIncrement' => null],
        '_indexes' => [
            'turn_id' => ['type' => 'index', 'columns' => ['turn_id'], 'length' => []],
            'matchnation_id' => ['type' => 'index', 'columns' => ['matchnation_id'], 'length' => []],
        ],
        '_constraints' => [
            'primary' => ['type' => 'primary', 'columns' => ['id'], 'length' => []],
            'matchnationturns_ibfk_1' => ['type' => 'foreign', 'columns' => ['turn_id'], 'references' => ['turns', 'id'], 'update' => 'restrict', 'delete' => 'restrict', 'length' => []],
            'matchnationturns_ibfk_2' => ['type' => 'foreign', 'columns' => ['matchnation_id'], 'references' => ['matchnations', 'id'], 'update' => 'restrict', 'delete' => 'restrict', 'length' => []],
        ],
        '_options' => [
            'engine' => 'InnoDB',
            'collation' => 'utf8_general_ci'
        ],
    ];
    // @codingStandardsIgnoreEnd

    /**
     * Records
     *
     * @var array
     */
    public $records = [
        [
            'id' => 1,
            'matchnation_id' => 1,
            'turn_id' => 1
        ],
    ];
}
