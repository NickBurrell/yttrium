using System;
using PanelId = System.UInt32;

namespace Yttrium
{
	public static class PanelManager
	{
		public static PanelId InitializePanel(UInt32 width, UInt32 height)
		{
			return 0;
		}

		public static void FreePanel(PanelId id)
		{
			return;
		}

		public static IntPtr GetPanelUpdateCallback(uint id)
		{
			return IntPtr.Zero;
		}

		public static void ResizePanel(UInt32 width, UInt32 height)
		{

		}

	}
}