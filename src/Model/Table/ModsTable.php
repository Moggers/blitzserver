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
		
        $this->hasMany('Nations', [
            'foreignKey' => 'mod_id'
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
			switch( $entity->get('Archive')['type']) {
				case "application/x-rar":
					break;
				case "application/zip":
				case "application/octet-stream":
				case "application/x-zip-compressed":
				case "application/x-zip":
					$zip = new \ZipArchive();
					$zip->open( $entity->get('Archive')['tmp_name'] );
					for( $i = 0; $i < $zip->numFiles; $i++ ) {
						$filepath = pathinfo($zip->getNameIndex($i));
						if( $filepath['filename'] == $entity->get('dmname')){
							$fd = fopen( WWW_ROOT . 'tmp/mods/' . $zip->getNameIndex($i), 'r' );
							$entity->set('dmname', $zip->getNameIndex($i));
							if( $fd ) {
								rewind( $fd );
								while( ( $line = fgets( $fd ) ) !== false ) {
									if( strpos( $line, '#disableoldnations' ) !== false ) {
										$entity->set( 'disableoldnations', 1 );
									}
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
							$zip->close();
							return true;
						}
					}
					break;
			}
			return true;
		}
	}

	public function afterSave( $event, $entity, $options )
	{
		if( $entity->get('Archive')['tmp_name'] !== '' ) {
			$moddir = DOM4_MODS . '/' . $entity->id . '/';
			if( !file_exists( $moddir ) )
				mkdir( $moddir, 0777, true );

			$filetmp = $entity->get('Archive')['tmp_name'];
			switch( $entity->get('Archive')['type'] ) {
				case 'application/x-rar':
					break;
				case "application/octet-stream":
				case "application/x-zip-compressed":
				case "application/x-zip":
				case 'application/zip':
					$fd = fopen( WWW_ROOT . 'tmp/mods/' . $entity->get('dmname'), 'r' );
					if( $fd ) {
						rewind( $fd );
						$nation = null;
						while( ( $line = fgets( $fd ) ) != false ) {
							if( strpos($line, '#end') !== false ) { 
								if( $nation != null ) {
									$this->Nations->save($nation);
									$nation = null;
								}
								continue;
							}
							$arr = explode( ' ', $line );
							switch( $arr[0] ) {
								case '--':
									break;
								case '#selectnation':
									$nation = $this->Nations->find('all')->where(['mod_id' => $entity->id, 'dom_id' => $arr[1]])->first();
									if( $nation == null ) { 
										$nation = $this->Nations->newEntity();
										$nation->mod_id = $entity->id;
										$nation->dom_id = $arr[1];
									}
									break;
								case '#era':
									if( $nation != null ) {
										$nation->age = $arr[1];
									}
									break;
								case '#name':
									if( $nation != null ) {
										$nation->name = $arr[1];
									}
									break;
								case '#epithet':
									if( $nation != null ) {
										$nation->subtitle = substr( $line, 10, -2 );
									}
									break;
							}
						}
						fclose( $fd );
					}
					$zip = new \ZipArchive();
					$zip->open( $entity->get('Archive')['tmp_name'] );
					$zip->extractTo( $moddir );
					$zip->close();
					break;
			}
			copy( $entity->get('Archive')['tmp_name'], $moddir . $entity->get('Archive')['name'] );

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
