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
		$query = $this->Mods->find('all')->where(['hidden' => 0]);
        $this->set('mods', $query );
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
        if ($this->request->is('post')) {
			$mc = 0;
			$mc1 = 0;
			switch( $this->request->data('Archive')['type']) {
				case "application/x-rar":
					break;
				case "application/octet-stream":
				case "application/x-zip":
				case "application/x-zip-compressed":
				case "application/zip":
					$zip = new \ZipArchive();
					$zip->open( $this->request->data('Archive')['tmp_name'] );
					for( $i = 0; $i < $zip->numFiles; $i++ ) {
						$filepath = pathinfo( $zip->getNameIndex($i) );
						if( isset( $filepath['extension'] ) && $filepath['extension'] == 'dm' && $filepath['dirname'] == '.' ) {
							$mod = $this->Mods->newEntity();
							$mod = $this->Mods->patchEntity($mod, $this->request->data);
							$mod->dmname = $filepath['basename'];
							if( !file_exists( 'tmp/mods/' ) )
								mkdir( 'tmp/mods/', 0777, true );
							$zip->extractTo( WWW_ROOT . 'tmp/mods/', $zip->getNameIndex($i) );
							$fd = fopen( WWW_ROOT . 'tmp/mods/' . $zip->getNameIndex($i), 'r' );
							$hash = crc32( fread($fd, 99999999 ) );
							$mod->crc32 = $hash;
							$clash = $this->Mods->find( 'all')->where(['crc32' => $hash ])->first();
							if( $clash ) {
								$mc1++;
							} else {
								fclose( $fd );
								if ($this->Mods->save($mod)) {
									$mc++;
								} else {
								}
							}
						}
					}
					$zip->close();
					break;
				case "7z":
					break;
			}
			$this->Flash->success(__($mc.' mods found and saved.'.($mc1 == 0?"":' '.$mc1." mods had already been uploaded")));
			return $this->redirect(['action' => 'index']);
        }
		$mod = $this->Mods->newEntity();
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

