<?php
namespace App\Model\Table;

use App\Model\Entity\Mod;
use Cake\ORM\Query;
use Cake\ORM\RulesChecker;
use Cake\ORM\Table;
use Cake\Validation\Validator;

/**
 * Mods Model
 *
 */
class ModsTable extends Table
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

        $this->table('mods');
        $this->displayField('name');
        $this->primaryKey('id');

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
            ->allowEmpty('icon');

        $validator
            ->allowEmpty('version');

        $validator
            ->allowEmpty('description');

        return $validator;
    }

	public function beforeSave( $event, $entity, $options )
	{
		if( $entity->get('Archive')['tmp_name'] != '' ) {
			$rar_file = rar_open( $entity->get('Archive')['tmp_name'] );
			$entries = rar_list($rar_file);
			foreach( $entries as $entry ) {
				$filepath = pathinfo( $entry->getName() );
				if( isset( $filepath['extension'] ) && $filepath['extension'] == 'dm' && $filepath['dirname'] == '.' ) {
					$fd = fopen( WWW_ROOT . 'mods/tmp/' . $entry->getName(), 'r' );
					$entity->set('dmname', $entry->getName());
					if( $fd ) {
						rewind( $fd );
						while( ( $line = fgets( $fd ) ) !== false ) {
							$arr = explode( ' ', $line );
							switch( $arr[0] ) {
								case '--':
									break;
								case '#version':
									$entity->set( 'version', trim( substr( $line, strlen('#version ' ) ) ) );
									break;
								case '#description':
									$entity->set( 'description', trim( str_replace( '"', '', substr( $line, strlen( '#description ' ) ) ) ) );
									break;
								case '#modname':
									$entity->set( 'name', trim( str_replace( '"', '', substr( $line, strlen( '#modname ' ) ) ) ) );
									break;
								case '#icon':
									$entity->set( 'icon', trim( str_Replace( '"', '', substr( $line, strlen( '#icon ' ) ) ) ) );
									break;
							}
						}
						fclose( $fd );
					}
					return true;
				}
			}
		}
		return true;
	}

	public function afterSave( $event, $entity, $options )
	{
		if( $entity->get('Archive')['tmp_name'] !== '' ) {
			$moddir = DOM4_MODS . '/' . $entity->id . '/';
			if( !file_exists( $moddir ) )
				mkdir( $moddir, 0777, true );

			$filetmp = $entity->get('Archive')['tmp_name'];
			$rar = rar_open( $entity->get('Archive')['tmp_name'] );
			foreach( rar_list($rar) as $entry ) {
				$entry->extract( $moddir );
			}
			$rar->close();
			move_uploaded_file( $entity->get('Archive')['tmp_name'], $moddir . $entity->get('Archive')['name'] );

			$thumbdir = WWW_ROOT . 'img/mods/' . $entity->id . '/';
			$thumbname = $thumbdir . "thumb64.jpeg";
			$iconname = DOM4_MODS . '/' . $entity->id . '/' . $entity->icon;
			if( !file_exists( $thumbdir ) )
				mkdir( $thumbdir, 0777, true );
			system( 'convert "' . $iconname . '" -scale -1x64 "' . $thumbname . '"' );
			return true;
		}
		else {
			return false;
		}
	}
}
