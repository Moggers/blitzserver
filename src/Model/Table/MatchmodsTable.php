<?php
namespace App\Model\Table;

use App\Model\Entity\Matchmod;
use Cake\ORM\Query;
use Cake\ORM\RulesChecker;
use Cake\ORM\Table;
use Cake\Validation\Validator;

/**
 * Matchmods Model
 *
 * @property \Cake\ORM\Association\BelongsTo $Matches
 * @property \Cake\ORM\Association\BelongsTo $Mods
 */
class MatchmodsTable extends Table
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

        $this->table('matchmods');
        $this->displayField('id');
        $this->primaryKey('id');

        $this->belongsTo('Matches', [
            'foreignKey' => 'match_id',
            'joinType' => 'INNER'
        ]);
        $this->belongsTo('Mods', [
            'foreignKey' => 'mod_id',
            'joinType' => 'INNER'
        ]);
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
            ->add('id', 'valid', ['rule' => 'numeric'])
            ->allowEmpty('id', 'create');

        return $validator;
    }

    /**
     * Returns a rules checker object that will be used for validating
     * application integrity.
     *
     * @param \Cake\ORM\RulesChecker $rules The rules object to be modified.
     * @return \Cake\ORM\RulesChecker
     */
    public function buildRules(RulesChecker $rules)
    {
        $rules->add($rules->existsIn(['match_id'], 'Matches'));
        $rules->add($rules->existsIn(['mod_id'], 'Mods'));
        return $rules;
    }
}
