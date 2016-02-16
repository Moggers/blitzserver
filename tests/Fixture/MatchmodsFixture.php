<?php
namespace App\Test\Fixture;

use Cake\TestSuite\Fixture\TestFixture;

/**
 * MatchmodsFixture
 *
 */
class MatchmodsFixture extends TestFixture
{

    /**
     * Fields
     *
     * @var array
     */
    // @codingStandardsIgnoreStart
    public $fields = [
        'id' => ['type' => 'integer', 'length' => 11, 'unsigned' => false, 'null' => false, 'default' => null, 'comment' => '', 'autoIncrement' => true, 'precision' => null],
        'match_id' => ['type' => 'integer', 'length' => 11, 'unsigned' => false, 'null' => false, 'default' => null, 'comment' => '', 'precision' => null, 'autoIncrement' => null],
        'mod_id' => ['type' => 'integer', 'length' => 11, 'unsigned' => false, 'null' => false, 'default' => null, 'comment' => '', 'precision' => null, 'autoIncrement' => null],
        '_indexes' => [
            'match_id' => ['type' => 'index', 'columns' => ['match_id'], 'length' => []],
            'mod_id' => ['type' => 'index', 'columns' => ['mod_id'], 'length' => []],
        ],
        '_constraints' => [
            'primary' => ['type' => 'primary', 'columns' => ['id'], 'length' => []],
            'matchmods_ibfk_1' => ['type' => 'foreign', 'columns' => ['match_id'], 'references' => ['matches', 'id'], 'update' => 'restrict', 'delete' => 'restrict', 'length' => []],
            'matchmods_ibfk_2' => ['type' => 'foreign', 'columns' => ['mod_id'], 'references' => ['mods', 'id'], 'update' => 'restrict', 'delete' => 'restrict', 'length' => []],
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
            'match_id' => 1,
            'mod_id' => 1
        ],
    ];
}
