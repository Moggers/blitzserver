<?php
namespace App\Controller;

use App\Controller\AppController;

/**
 * Mods Controller
 *
 * @property \App\Model\Table\ModsTable $Mods
 */
class ModsController extends AppController
{

    /**
     * Index method
     *
     * @return void
     */
    public function index()
    {
        $this->set('mods', $this->paginate($this->Mods));
        $this->set('_serialize', ['mods']);
    }

    /**
     * View method
     *
     * @param string|null $id Mod id.
     * @return void
     * @throws \Cake\Network\Exception\NotFoundException When record not found.
     */
    public function view($id = null)
    {
        $mod = $this->Mods->get($id, [
            'contain' => []
        ]);
        $this->set('mod', $mod);
        $this->set('_serialize', ['mod']);
    }

    /**
     * Add method
     *
     * @return void Redirects on successful add, renders view otherwise.
     */
    public function add()
    {
        $mod = $this->Mods->newEntity();
        if ($this->request->is('post')) {
            $mod = $this->Mods->patchEntity($mod, $this->request->data);
			switch( $this->request->data('Archive')['type']) {
				case "application/x-rar":
					$rar_file = rar_open( $this->request->data('Archive')['tmp_name'] );
					$entries = rar_list($rar_file);
					foreach( $entries as $entry ) {
						$filepath = pathinfo( $entry->getName() );
						if( isset( $filepath['extension'] ) && $filepath['extension'] == 'dm' && $filepath['dirname'] == '.' ) {
							if( !file_exists( 'mods/tmp/' ) )
								mkdir( 'mods/tmp/', 0777, true );
							$entry->extract( WWW_ROOT . 'mods/tmp/' );
							$fd = fopen( WWW_ROOT . 'mods/tmp/' . $entry->getName(), 'r' );
							$rar_file->close();
							$hash = crc32( fread($fd, 99999999 ) );
							$mod->crc32 = $hash;
							$clash = $this->Mods->find( 'all')->where(['crc32' => $hash ])->first();
							if( $clash ) {
								$this->Flash->error(__('That map has already been uploaded. It\'s over here'));
								return $this->redirect(['action' => 'view', $clash->id]);
							}
							fclose( $fd );
						}
					}
					break;
				case "application/zip":
					$zip_file = zip_open( $this->request->data('Archive')['tmp_name'] );
					while( $entry = zip_read($zip_file)) {
						$filepath = pathinfo( zip_entry_name( $entry) );
						if( isset( $filepath['extension'] ) && $filepath['extension'] == 'dm' && $filepath['dirname'] == '.' ) {
							if( !file_exists( 'mods/tmp/' ) )
								mkdir( 'mods/tmp/', 0777, true );
							$zip_file->extractTo( WWW_ROOT . 'mods/tmp/', $entry );
							$fd = fopen( WWW_ROOT . 'mods/tmp/' . zip_entry_name( $entry ), 'r' );
							$zip_file->close();
							$hash = crc32( fread($fd, 99999999 ) );
							$mod->crc32 = $hash;
							$clash = $this->Mods->find( 'all')->where(['crc32' => $hash ])->first();
							if( $clash ) {
								$this->Flash->error(__('That map has already been uploaded. It\'s over here'));
								return $this->redirect(['action' => 'view', $clash->id]);
							}
							fclose( $fd );
						}
					}
					break;
				case "7z":
					break;
			}
            if ($this->Mods->save($mod)) {
                $this->Flash->success(__('The mod has been saved.'));
                return $this->redirect(['action' => 'index']);
            } else {
                $this->Flash->error(__('The mod could not be saved. Please, try again.'));
            }
        }
        $this->set(compact('mod'));
        $this->set('_serialize', ['mod']);
    }

    /**
     * Edit method
     *
     * @param string|null $id Mod id.
     * @return void Redirects on successful edit, renders view otherwise.
     * @throws \Cake\Network\Exception\NotFoundException When record not found.
     */
    public function edit($id = null)
    {
        $mod = $this->Mods->get($id, [
            'contain' => []
        ]);
        if ($this->request->is(['patch', 'post', 'put'])) {
            $mod = $this->Mods->patchEntity($mod, $this->request->data);
            if ($this->Mods->save($mod)) {
                $this->Flash->success(__('The mod has been saved.'));
                return $this->redirect(['action' => 'index']);
            } else {
                $this->Flash->error(__('The mod could not be saved. Please, try again.'));
            }
        }
        $this->set(compact('mod'));
        $this->set('_serialize', ['mod']);
    }

    /**
     * Delete method
     *
     * @param string|null $id Mod id.
     * @return \Cake\Network\Response|null Redirects to index.
     * @throws \Cake\Network\Exception\NotFoundException When record not found.
     */
    public function delete($id = null)
    {
        $this->request->allowMethod(['post', 'delete']);
        $mod = $this->Mods->get($id);
        if ($this->Mods->delete($mod)) {
            $this->Flash->success(__('The mod has been deleted.'));
        } else {
            $this->Flash->error(__('The mod could not be deleted. Please, try again.'));
        }
        return $this->redirect(['action' => 'index']);
    }
}
