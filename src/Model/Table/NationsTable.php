<?php
namespace App\Model\Table;

use App\Model\Entity\Nation;
use Cake\ORM\Query;
use Cake\ORM\RulesChecker;
use Cake\ORM\Table;
use Cake\Validation\Validator;

/**
 * Nations Model
 *
<<<<<<< HEAD
=======
 * @property \Cake\ORM\Association\HasMany $Matchnations
>>>>>>> Transitioned from bitstring based nation handling to a discrete table with a belongsToMany relationship
 */
class NationsTable extends Table
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

        $this->table('nations');
        $this->displayField('name');
        $this->primaryKey('id');

        $this->hasMany('Matchnations', [
            'foreignKey' => 'nation_id'
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

        $validator
            ->allowEmpty('name');

        $validator
            ->allowEmpty('subtitle');

        $validator
            ->allowEmpty('turn_name');

        return $validator;
    }
}
