<?php
namespace App\Controller;

use App\Controller\AppController;

/**
 * Maps Controller
 *
 * @property \App\Model\Table\MapsTable $Maps
 */
class MapsController extends AppController
{

    /**
     * Index method
     *
     * @return void
     */
    public function index()
    {
        $this->set('maps', $this->paginate($this->Maps->find('all', [ 
			'conditions' => [
				'Maps.hide =' => 0
			]
		])));
        $this->set('_serialize', ['maps']);
    }

    /**
     * View method
     *
     * @param string|null $id Map id.
     * @return void
     * @throws \Cake\Network\Exception\NotFoundException When record not found.
     */
    public function view($id = null)
    {
        $map = $this->Maps->get($id, [
            'contain' => []
        ]);
        $this->set('map', $map);
        $this->set('_serialize', ['map']);
    }

    /**
     * Add method
     *
     * @return void Redirects on successful add, renders view otherwise.
     */
    public function add()
    {
        $map = $this->Maps->newEntity();
        if ($this->request->is('post')) {
            $map = $this->Maps->patchEntity($map, $this->request->data);
			$imagetmp = $this->request->data('Definition')['tmp_name'];
			$fd = fopen( $imagetmp, 'r' );
			$hash = crc32( fread( $fd, 99999999 ) );
			$map->crc32 = $hash;
			$map->hide = 0;
			$clash = $this->Maps->find( 'all' )->where(['crc32'=> $hash])->first();
			if( $clash ) {
				$this->Flash->error(__('That map has already been uploaded. It\'s over here'));
				$this->redirect(['action' => 'view', $clash->id]);
			}
            if ($this->Maps->save($map)) {
				$thumbdir = WWW_ROOT . 'img/maps/' . $map->id . '/';
				if( !file_exists( $thumbdir ) )
					mkdir( $thumbdir, 0777, true );
				$thumbname = $thumbdir . "thumb64.jpeg";
				$rgbname =  DOM4_MAPS . '/' . $map->id . '/' . $map->imagepath;
				system( "convert \"" . $rgbname . "\" -scale 64x-1 \"" . $thumbname . "\"" );
				$thumbname = $thumbdir . "thumb512.jpeg";
				system( "convert \"" . $rgbname . "\" -scale 512x-1 \"" . $thumbname . "\"" );
                return $this->redirect(['action' => 'index']);
            } else {
                $this->Flash->error(__('The map could not be saved. Please, try again.'));
            }
        }
        $this->set(compact('map'));
        $this->set('_serialize', ['map']);
    }

    /**
     * Edit method
     *
     * @param string|null $id Map id.
     * @return void Redirects on successful edit, renders view otherwise.
     * @throws \Cake\Network\Exception\NotFoundException When record not found.
     */
    public function edit($id = null)
    {
        $map = $this->Maps->get($id, [
            'contain' => []
        ]);
        if ($this->request->is(['patch', 'post', 'put'])) {
            $map = $this->Maps->patchEntity($map, $this->request->data);
            if ($this->Maps->save($map)) {
                $this->Flash->success(__('The map has been saved.'));
                return $this->redirect(['action' => 'index']);
            } else {
                $this->Flash->error(__('The map could not be saved. Please, try again.'));
            }
        }
        $this->set(compact('map'));
        $this->set('_serialize', ['map']);
    }

    /**
     * Delete method
     *
     * @param string|null $id Map id.
     * @return \Cake\Network\Response|null Redirects to index.
     * @throws \Cake\Network\Exception\NotFoundException When record not found.
     */
    public function delete($id = null)
    {
        $this->request->allowMethod(['post', 'delete']);
        $map = $this->Maps->get($id);
		$map->hide = 1;
		if ($this->Maps->save($map)) {
			$this->Flash->success(__('The map has been deleted.'));
			return $this->redirect(['action' => 'index']);
		} else {
			$this->Flash->error(__('The map could not be deleted. Please, try again.'));
		}
        return $this->redirect(['action' => 'index']);
    }
}
