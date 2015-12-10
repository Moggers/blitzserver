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
        $this->set('maps', $this->paginate($this->Maps));
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
            if ($this->Maps->save($map)) {
				//$pathpart = pathinfo( $map->imagepath, PATHINFO_FILENAME );
				//system( "convert " . $pathpart . ".rgb -resize 64x-1 " . $pathpart .".jpeg" );
				$imgname = WWW_ROOT . "uploads/maps/" . $map->id . "/" . pathinfo( $map->imagepath, PATHINFO_FILENAME ) . ".jpeg";
				system( "convert " . $map->imagepath . " " . $imgname );
				$mapdir = WWW_ROOT . 'img/maps/' . $map->id;
				if( !file_exists( $mapdir ) )
					mkdir( $mapdir, 0777, true );
				$img = new \Imagick();
				$handle = fopen($imgname, 'r' );
				$img->readImageFile( $handle, $imgname );
				$img->resizeImage( 512, -1, 0, 1 );
				$img->writeImages( WWW_ROOT . 'img/maps/' . $map->id . '/thumb128.jpeg', false );
				$img->resizeImage( 64, -1, 0, 1 );
				$img->writeImages( WWW_ROOT . 'img/maps/' . $map->id . '/thumb64.jpeg', false );
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
        if ($this->Maps->delete($map)) {
            $this->Flash->success(__('The map has been deleted.'));
        } else {
            $this->Flash->error(__('The map could not be deleted. Please, try again.'));
        }
        return $this->redirect(['action' => 'index']);
    }
}
