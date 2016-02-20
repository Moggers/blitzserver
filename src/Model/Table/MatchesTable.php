<?php
namespace App\Model\Table;

use App\Model\Entity\Match;
use Cake\ORM\Query;
use Cake\ORM\RulesChecker;
use Cake\ORM\Table;
use Cake\Validation\Validator;

/**
 * Matches Model
 *
 * @property \Cake\ORM\Association\BelongsTo $Maps
 * @Property \Cake\ORM\Association\BelongsToMany $Nations
 * @Property \Cake\ORM\Association\BelongsToMany $Mods
 */
class MatchesTable extends Table
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

        $this->table('matches');
        $this->displayField('name');
        $this->primaryKey('id');

		$this->hasMany( 'Turns', [
			'foreignKey' => 'match_id',
			'joinType' => 'INNER'
		]);
        $this->belongsTo('Maps', [
            'foreignKey' => 'map_id',
            'joinType' => 'INNER'
        ]);
		$this->belongsToMany( 'Nations', [
			'targetForeignKey' => 'nation_id',
			'joinTable' => 'matchnations',
			'joinType' => 'INNER'
		]);

		$this->belongsToMany( 'Mods', [
			'targetForeignKey' => 'mod_id',
			'joinTable' => 'matchmods',
			'joinType' => 'INNER'
		]);
    }

	public function afterFind( $results, $primary = false )
	{
		die( "Just fuck my shit up fam" );
		foreach( $results as $key => $val ) {
			if( isset( $val['Match']['name'] ) ) {
				$results[$key]['Match']['name'] = str_replace( '_', ' ', $results[$key]['Match']['name'] );
			}
		}
		return $results;
	}

	public function beforeSave( $options = array() )
	{
		if( !empty($entity->match['name']) ) {
			$entity->match['name'] = str_replace( ' ', '_', $entity->match['name'] );
		}
		return true;
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
            ->add('age', 'valid', ['rule' => 'numeric'])
            ->allowEmpty('age');

		$validator
			->add('name', 'valid', ['rule' => 'notBlank']);

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
        $rules->add($rules->existsIn(['map_id'], 'Maps'));
        return $rules;
    }
}
