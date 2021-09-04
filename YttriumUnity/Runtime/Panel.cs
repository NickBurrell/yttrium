using System;
using UnityEngine;
using UnityEngine.Events;
using UnityEngine.Rendering;

using PanelId = System.UInt32;

namespace Yttrium
{
	public class Panel : MonoBehaviour
	{

		private PanelId _id;

		private Texture2D     _texture;
		private CommandBuffer _command;

		public                  int Width;
		public                  int Height;

		private static readonly int MainTex = Shader.PropertyToID("_MainTex");

		private void Start()
		{
			_id = PanelManager.InitializePanel();

			_command = new CommandBuffer();
			_texture = new Texture2D(Width, Height, TextureFormat.RGBA32, false)
			{
				wrapMode = TextureWrapMode.Clamp
			};

			var prop = new MaterialPropertyBlock();
			prop.SetTexture(MainTex, _texture);
			GetComponent<Renderer>().SetPropertyBlock(prop);

		}

		private void OnDestroy()
		{
			_command.Dispose();
			Destroy(_texture);
			PanelManager.FreePanel(_id);
		}

		private void Update()
		{
			Draw();
		}

		#region Internal

		private void Draw()
		{
			_command.IssuePluginCustomTextureUpdateV2(PanelManager.GetPanelUpdateCallback(_id), _texture, 0);
			Graphics.ExecuteCommandBuffer(_command);
			_command.Clear();
		}

		private void ClickHandler()
		{
			// TODO: Add callback support for compatibility
			if (Input.GetMouseButton(0))
			{
				if (Camera.main is { } &&
				    Physics.Raycast(Camera.main.ScreenPointToRay(Input.mousePosition), out var hit) &&
				    hit.collider.gameObject == gameObject)
				{
					Debug.Log(hit);
				}
			}
		}

		#endregion
	}
}