<?php
namespace App\Model\Table;

use App\Model\Entity\Map;
use Cake\ORM\Query;
use Cake\ORM\RulesChecker;
use Cake\ORM\Table;
use Cake\Validation\Validator;

/**
 * Maps Model
 *
 */
class MapsTable extends Table
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

        $this->table('maps');
        $this->displayField('name');
        $this->primaryKey('id');

    }

    /**
     * Default validation rules.
     *
     * @param \Cake\Validation\Validator $validator Validator instance.
     * @return \Cake\Validation\Validator
     */

	 public function beforeSave( $event, $entity, $options )
	 {
		 $imagetmp = $entity->get('Definition')['tmp_name'];
		 $fd = fopen( $imagetmp, 'r' );
		 $entity->set( 'imagepath', $entity->get('Image')['name'] );
		 if( $fd ) {
			$terraincount = 0;
			$watercount = 0;
			 while( ( $line = fgets( $fd ) ) !== false ) {
				 $arr = explode( ' ', $line );
				 if( $arr[0] == '--' ) {
				 }
				 else if( $arr[0] == '#dom2title' ) {
					 $entity->set( 'name', substr( $line, 10 ) );
				 }
				 else if( $arr[0] == "#description" ) {
					 $entity->set( 'description', substr( $line, 14 ) );
				 }
				 else if( $arr[0] == "#terrain" ) {
					$terraincount++;
					$water = intval( $arr[2] ) & 4;
					if( $water == 8 || $water == 4 || $water == 12 ) 
						$watercount++;
				 }
				 else if( $arr[0] == "#hwraparound" ) {
					 $entity->set( 'hwrap', true );
				 }
				 else if( $arr[0] == "#vwraparound" ) {
					 $entity->set( 'vwrap', true );
				 }
			 }
			 $entity->set( 'prov', $terraincount );
			 $entity->set( 'seaprov', $watercount );
			 fclose( $fd );
		 }
		 else {
			 return false;
		 }

		 $entity->set( 'mappath', $entity->get('Definition')['name'] );
	 return true;
	 }
	 public function afterSave( $event, $entity, $options )
	 {
		 $mapdir = WWW_ROOT . 'uploads/maps/' . $entity->id . '/';
		 if( !file_exists( $mapdir ) )
			 mkdir( $mapdir, 0777, true );

		 $imagetmp = $entity->get('Image')['tmp_name'];
		 $imagepath = $mapdir . $entity->get('Image')['name'];
		 move_uploaded_file( $imagetmp, $imagepath );

		 $deftmp = $entity->get('Definition')['tmp_name'];
		 $defpath = $mapdir . $entity->get('Definition')['name'];
		 move_uploaded_file( $deftmp, $defpath );
	 }

    public function validationDefault(Validator $validator)
    {
        $validator
            ->add('id', 'valid', ['rule' => 'numeric'])
            ->allowEmpty('id', 'create');

		// What the fuck
        /*$validator-> add('Image', 'file', [
				'rule' => ['mimeType', ['image/x-tga,image/x-rgb']],
				'message' => 'RGB or TGA only, please' ]);*/

        return $validator;
    }
}
