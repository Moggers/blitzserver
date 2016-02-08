<?php
namespace App\Model\Table;

use App\Model\Entity\Matchnation;
use Cake\ORM\Query;
use Cake\ORM\RulesChecker;
use Cake\ORM\Table;
use Cake\Validation\Validator;

/**
 * Matchnations Model
 *
 */
class MatchnationsTable extends Table
{

    /**
     * Initialize method
     *
     * @param array $config The configuration for the Table.
     * @return void
     */
    public function initialize(array $config)
    {
        parent::initialize($config);

        $this->table('matchnations');
        $this->displayField('matchid');
        $this->primaryKey(['matchid', 'nationid']);

    }

    /**
     * Default validation rules.
     *
     * @param \Cake\Validation\Validator $validator Validator instance.
     * @return \Cake\Validation\Validator
     */
    public function validationDefault(Validator $validator)
    {
        $validator
            ->add('matchid', 'valid', ['rule' => 'numeric'])
            ->allowEmpty('matchid', 'create');

        $validator
            ->add('nationid', 'valid', ['rule' => 'numeric'])
            ->allowEmpty('nationid', 'create');

        $validator
            ->add('markdelete', 'valid', ['rule' => 'numeric'])
            ->allowEmpty('markdelete');

        return $validator;
    }
}
