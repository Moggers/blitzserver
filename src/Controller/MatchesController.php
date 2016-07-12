<?php
namespace App\Controller;

use App\Controller\AppController;
use Cake\Controller\Component\CookieComponent;

/**
 * Matches Controller
 * * @property \App\Model\Table\MatchesTable $Matches
 */
class MatchesController extends AppController
{

	/**
	 * Paginator
	 */

	/**
	 * Index method
	 *
	 * @return void
	 */
	public function index()
	{
		$this->paginate = [
			'contain' => ['Maps', 'Nations', 'Turns'],
			'limit' => 10000,
			'maxLimit' => 10000
		];

		if( $this->request->query('layout') == 'false' ) {
			$this->viewBuilder()->layout( false );
		}
		$this->set('lobbymatches', $this->paginate($this->Matches->find()->where(['status IN' => [0,1,-2]])));
		$this->set('finishedmatches', $this->paginate($this->Matches->find()->where(['status IN' => [-1,70,71]])));
		$this->set('progressmatches', $this->paginate($this->Matches->find()->where(['status IN' => [3,2]])));
		$this->set('_serialize', ['lobbymatches']);
		$this->set('_serialize', ['finishedmatches']);
		$this->set('_serialize', ['progressmatches']);
	}

	/**
	 * SCHEDULE MEMES
	 */
	public function weekschedule( $id = null )
	{
		$match = $this->Matches->get($id );
		if( isset( $_COOKIE['password']) && $match->checkPassword( $_COOKIE['password'] )){
			if( $this->request->is('post') || $this->request->is('put')) {
				$match->day = $this->request->data['day'];
				$match->hour = $this->request->data['hour'];
				if( $this->Matches->save($match)) {
					$this->Flash->success(__('Schedule updated'));
				} else {
					$this->Flash->error('Failed to update schedule');
				}
			} else { die( pr($this) ); }
			die( json_encode( [ 'status' => 1, 'id' => $match->id ] ) );
		} else {
			die( json_encode( [ 'status' => 0, 'id' => $match->id ] ) );
		}
	}

	public function markcomputer( $id = null, $nation_id = null )
	{
		$match = $this->Matches->get($this->request->data['id'],[
			'contain' => ['Nations']
			]);
		if( isset( $_COOKIE['password']) && $match->checkPassword( $_COOKIE['password'] )){
			$conditions = ['match_id' => $this->request->data['id'], 'nation_id' => $this->request->data['nation_id']];
			if( $this->Matches->Matchnations->find('all')->where($conditions)->isEmpty()) {
				$matchnation = $this->Matches->Matchnations->newEntity();
			} else {
				$matchnation = $this->Matches->Matchnations->find('all')->where( $conditions );
			}
			$matchnation->match_id = $this->request->data['id'];
			$matchnation->nation_id = $this->request->data['nation_id'];
			$matchnation->computer = 1;
			if( $this->Matches->Matchnations->save($matchnation) ) {
				die( json_encode( [ 'status' => 0, 'id' => $match->id ] ) );
			} else {
				die( json_encode( [ 'status' => 2, 'id' => $match->id ] ) );
			}
		} else {
			die( json_encode( [ 'status' => 1, 'id' => $match->id ] ) );
		}
	}
	public function turndelay( $id = null, $turndelay = null)
	{
		$match = $this->Matches->get($this->request->data['id']);
		if( isset( $_COOKIE['password']) && $match->checkPassword( $_COOKIE['password'] )){
			$match->turndelay = $this->request->data['turndelay'];
			if( $this->Matches->save($match) ) {
				die( json_encode( [ 'status' => 0, 'id' => $match->id ] ) );
			} else {
				die( json_encode( [ 'status' => 2, 'id' => $match->id ] ) );
			}
		} else {
			die( json_encode( [ 'status' => 1, 'id' => $match->id ] ) );
		}
	}

	public function hostinterval( $id = null )
	{
		$match = $this->Matches->get($id );
		if( isset( $_COOKIE['password']) && $match->checkPassword( $_COOKIE['password'] )){
			if( $this->request->is('post') || $this->request->is('put')) {
				$match->hostinterval = $this->request->data['hour'] * 60 + $this->request->data['minute'];
				if( $this->Matches->save($match)) {
					$this->Flash->success(__('Schedule updated'));
				} else {
					$this->Flash->error('Failed to update schedule');
				}
			} else { die( pr($this) ); }
			die( json_encode( [ 'status' => 1, 'id' => $match->id ] ) );
		} else {
			die( json_encode( [ 'status' => 0, 'id' => $match->id ] ) );
		}
	}

	/**
	 * View method
	 *
	 * @param string|null $id Match id.
	 * @return void
	 * @throws \Cake\Network\Exception\NotFoundException When record not found.
	 */
	public function view($id = null)
	{
		$match = $this->Matches->get($id, [
				'contain' => ['Maps', 'Nations', 'Mods', 'Turns', 'Turns.Matchnationturns', 'Posts' => ['sort' =>['Posts.id' => 'DESC']]]
		]);
		$maps = $this->Matches->Maps->find('list', ['limit' => 200]);
		$this->set(compact('match', 'maps'));
		$nations = $this->Matches->Matchnations->find('list', ['keyField' => 'id', 'valueField' => 'nation.name'])->where(['match_id' => $id])->contain(['Nations']);
		$this->loadModel('Nations');
		$ids = array();
		$disableold = 0;
		for( $i=0; $i < count($match->mods); $i++ ) {
			if( $match->mods[$i]->disableoldnations == 1 )
				$disableold = 1;
			if( $match->mods[$i]->id != 1 || $disableold == 0 )
				array_push( $ids, $match->mods[$i]->id );
		}
		$allnations = $this->Nations->find('list', ['keyField' => 'id', 'valueField' => 'name'])->where(['age' => $match->age, 'mod_id IN' => $ids]);
		$this->set(compact('match', 'nations'));
		$this->set(compact('match','allnations'));
		$this->set('match', $match);
		$newpost = $this->Matches->Posts->newEntity();
		$newpost->match_id = $match->id;
		$this->set('newpost', $newpost );
		$this->set('_serialize', ['match']);
		$this->set('mods', $this->paginate(
			$this->Matches->Mods
			->find()
			->matching('Matches', function(\Cake\ORM\Query $q ) use ($id) {
				return $q->where([
					'Matches.id' => $id
				]);
			})->group(['Mods.id'])
		));
	}

	/**
	 * Add method
	 *
	 * @return void Redirects on successful add, renders view otherwise.
	 */
	public function add()
	{
		$match = $this->Matches->newEntity();
		if ($this->request->is('post')) {
			$match = $this->Matches->patchEntity($match, $this->request->data, ['associated' => ['Mods']]);
			if( isset($_COOKIE['password']) && $_COOKIE['password'] != '' ){
				if( $match->name != '' ) 
					$match->name = str_replace( ' ', '_', $match->name );
				$match->status = 0;
				$match->password = $_COOKIE['password'];
				$match->port = 0;
				// Insert vanilla mod to link all games to vanilla nations
				array_push($match->mods, $this->Matches->Mods->find('all')->where(['id' => 1])->first());
				if ($this->Matches->save($match)) {
					die( json_encode( [ 'status' => 0, 'id' => $match->id ] ) );
				} else {
					die( json_encode( [ 'status' => 2, 'id' => $match->id ] ) );
				}
			} else {
				die( json_encode( [ 'status' => 1, 'id' => $match->id ] ) );
			}
		}
		$maps = $this->Matches->Maps->find('all', ['limit' => 200])->where(['hide' => 0]);
		$this->set(compact('match', 'maps'));
		$this->set('modsfull', $this->Matches->Mods->find()->where(['hidden' => 0]));
		$this->set('mods', $this->Matches->Mods->find('list') );
		$this->set('_serialize', ['match']);
	}

	/**
	 * Edit method
	 *
	 * @param string|null $id Match id.
	 * @return void Redirects on successful edit, renders view otherwise.
	 * @throws \Cake\Network\Exception\NotFoundException When record not found.
	 */
	public function edit($id = null)
	{
		$match = $this->Matches->get($id, [
				'contain' => []
		]);
		if ($this->request->is(['patch', 'post', 'put'])) {
			if( isset( $_COOKIE['password']) && $match->checkPassword( $_COOKIE['password'] )){
				$match = $this->Matches->patchEntity($match, $this->request->data);
				$match->needsrestart = 1;
				if ($this->Matches->save($match)) {
					$this->Flash->success(__('The match has been saved. The server will now restart to apply your changes'));
					return $this->redirect(['action' => 'view', $match->id]);
				} else {
					$this->Flash->error(__('The match could not be saved. Please, try again.'));
				}
			} else {
				$this->Flash->error(__('Incorrect password :)'));
			}
		}
		$maps = $this->Matches->Maps->find('list', ['limit' => 200]);
		$this->set(compact('match', 'maps'));
		$this->set('_serialize', ['match']);
		return $this->redirect(['action' => 'view', $match->id]);
	}

	/**
	 * Delete method
	 *
	 * @param string|null $id Match id.
	 * @return \Cake\Network\Response|null Redirects to index.
	 * @throws \Cake\Network\Exception\NotFoundException When record not found.
	 */
	public function delete($id = null)
	{
		$this->request->allowMethod(['post', 'delete']);
		$match = $this->Matches->get($id);
		if ($this->Matches->delete($match)) {
			$this->Flash->success(__('The match has been deleted.'));
		} else {
			$this->Flash->error(__('The match could not be deleted. Please, try again.'));
		}
		return $this->redirect(['action' => 'index']);
	}

	public function start($id = null)
	{
		$match = $this->Matches->get($id, [
				'contain' => []
		]);
		if ($this->request->is(['patch', 'get', 'put'])) {
			// Get us out of here
			$this->Flash->error(__('Starting matches through this interface is currently broken. Please enable Client Start, and begin the match from inside the game'));
			return $this->redirect(['action' => 'index']);
			$match = $this->Matches->patchEntity($match, $this->request->data);

			// Once I fix this bullshit
			if( $match->playerstring == 0 ) {
				$this->Flash->error( "Can't start a game without players" );
				return $this->redirect(['action' => 'view', $match->id]);
			}
			$match->status = 2;
			if ($this->Matches->save($match)) {
				$this->Flash->success(__('The match has been started.'));
				return $this->redirect(['action' => 'index']);
			} else {
				$this->Flash->error(__('Oh fuck'));
			}
		}
		$maps = $this->Matches->Maps->find('list', ['limit' => 200]);
		$this->set(compact('match', 'maps'));
		$this->set('_serialize', ['match']);
	}
	public function finish($id = null)
	{
		$match = $this->Matches->get($id, [
				'contain' => []
		]);
		if ($this->request->is(['patch', 'get', 'put'])) {
			if( isset( $_COOKIE['password']) && $match->checkPassword( $_COOKIE['password'] )){
				$match = $this->Matches->patchEntity($match, $this->request->data);
				$match->status = 71;
				if ($this->Matches->save($match)) {
					die( json_encode( ['status' => 0] ) );
				} else {
					die( json_encode( ['status' => 2] ) );
				}
			} else {
				die( json_encode( ['status' => 1] ) );
			}
		}
		$maps = $this->Matches->Maps->find('list', ['limit' => 200]);
		$this->set(compact('match', 'maps'));
		$this->set('_serialize', ['match']);
		die( json_encode( ['status' => 2] ) );
	}

	public function unstart($id = null)
	{
		$match = $this->Matches->get($id, [
				'contain' => []
		]);
		if ($this->request->is(['patch', 'get', 'put'])) {
			if( isset( $_COOKIE['password']) && $match->checkPassword( $_COOKIE['password'] )){
				$match = $this->Matches->patchEntity($match, $this->request->data);
				$match->status = -2;
				if ($this->Matches->save($match)) {
					die( json_encode( ['status' => 0] ) );
				} else {
					die( json_encode( ['status' => 2] ) );
				}
			} else {
				die( json_encode( ['status' => 1] ) );
			}
		}
		$maps = $this->Matches->Maps->find('list', ['limit' => 200]);
		$this->set(compact('match', 'maps'));
		$this->set('_serialize', ['match']);
		die( json_encode( ['status' => 2] ) );
	}
	public function destroy($id = null)
	{
		$match = $this->Matches->get($id, [
				'contain' => []
		]);
		if ($this->request->is(['patch', 'get', 'put'])) {
			if( isset( $_COOKIE['password']) && $match->checkPassword( $_COOKIE['password'] )){
				$match = $this->Matches->patchEntity($match, $this->request->data);
				$match->status = -1;
				if ($this->Matches->save($match)) {
					die( json_encode( ['status' => 0] ) );
				} else {
					die( json_encode( ['status' => 2] ) );
				}
			} else {
				die( json_encode( ['status' => 1] ) );
			}
		}
		$maps = $this->Matches->Maps->find('list', ['limit' => 200]);
		$this->set(compact('match', 'maps'));
		$this->set('_serialize', ['match']);
		die( json_encode( ['status' => 2] ) );
	}

	public function removePlayer( $id = null )
	{
		if( $this->request->is(['patch', 'get', 'put']) ) {
			$matchnation = $this->Matches->Nations->Matchnations->get($id, [
					'contain' => []
			]);
			$match = $this->Matches->get( $matchnation->match_id, [
					'contain' => []
			]);
			if( isset( $_COOKIE['password'] )&& $match->checkPassword( $_COOKIE['password'])){
				if( $match->status !== 3 ) {
					$matchnation->markdelete = 1;
					if( $this->Matches->Nations->Matchnations->save($matchnation) ) {
						$this->Flash->success(__('NOTE: Due to limitations in Dom4\'s server CLI, removing a player requires the server to be restarted, and thus, all players will have to reconnect.'));
					} else {
						$this->Flash->error(__('Hamlet: O fuck') );
					}
				} else {
					$this->Flash->error(__('Can\'t remove players from an active game' ) );
				}
			} else {
				$this->Flash->error(__('Incorrect password' ) );
			}
		}
		return $this->redirect(['action' => 'view', $matchnation->match_id]);
	}

	public function mapview( $id = null, $turnid=null )
	{
		$match = $this->Matches->get($id, [
				'contain' => ['Maps', 'Nations', 'Mods', 'Turns']
		]);
		if( $turnid == null ) {
			$turnid = $match->turn->tn-1;
		}
		$this->set('match', $match);
		$this->set('_serialize', ['match']);
		$this->set('turnid', $turnid);
	}

	/*
	 * Request email notification
	 */
	public function requestnotify($id = null)
	{
		if( $this->request->is(['ajax']) ) {
			$emailrequest = $this->Matches->Emailrequests->newEntity();
			$emailrequest = $this->Matches->Emailrequests->patchEntity($emailrequest, $this->request->data);
			$emailrequest->match_id = $id;
			if( $this->Matches->Emailrequests->save( $emailrequest ) ) {
				die( json_encode( ['status' => 0, 'id' => $id ] ) );
			} else {
				die( json_encode( ['status' => 1, 'id' => $id ] ) );
			}
		}
	}
}
