<?php
namespace App\Test\Fixture;

use Cake\TestSuite\Fixture\TestFixture;

/**
 * MatchnationsFixture
 *
 */
class MatchnationsFixture extends TestFixture
{

    /**
     * Fields
     *
     * @var array
     */
    // @codingStandardsIgnoreStart
    public $fields = [
        'matchid' => ['type' => 'integer', 'length' => 11, 'unsigned' => false, 'null' => false, 'default' => null, 'comment' => '', 'precision' => null, 'autoIncrement' => null],
        'nationid' => ['type' => 'integer', 'length' => 11, 'unsigned' => false, 'null' => false, 'default' => null, 'comment' => '', 'precision' => null, 'autoIncrement' => null],
        'markdelete' => ['type' => 'integer', 'length' => 11, 'unsigned' => false, 'null' => true, 'default' => null, 'comment' => '', 'precision' => null, 'autoIncrement' => null],
        '_indexes' => [
            'nationid' => ['type' => 'index', 'columns' => ['nationid'], 'length' => []],
        ],
        '_constraints' => [
            'primary' => ['type' => 'primary', 'columns' => ['matchid', 'nationid'], 'length' => []],
            'matchnations_ibfk_1' => ['type' => 'foreign', 'columns' => ['matchid'], 'references' => ['matches', 'id'], 'update' => 'restrict', 'delete' => 'restrict', 'length' => []],
            'matchnations_ibfk_2' => ['type' => 'foreign', 'columns' => ['nationid'], 'references' => ['nations', 'id'], 'update' => 'restrict', 'delete' => 'restrict', 'length' => []],
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
            'matchid' => 1,
            'nationid' => 1,
            'markdelete' => 1
        ],
    ];
}
