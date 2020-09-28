using System;
using System.Net;
using System.Net.Sockets;
using System.Text;

namespace UdpPing
{
	internal class Program
	{
		private static string IpAdress = "127.0.0.1";

		public static void Main(string[] args)
		{
			UdpClient udpClient = new UdpClient();
			udpClient.Client.Bind(new IPEndPoint(IPAddress.Parse(IpAdress), 9877));

			IPEndPoint ipEndPoint = new IPEndPoint(IPAddress.Parse(IpAdress), 9878);
			string input;

			
			while (true)
			{
				input = Console.ReadLine();
				var byteNames = Encoding.UTF8.GetBytes(input);
				udpClient.Send(byteNames, byteNames.Length, ipEndPoint);
				if (input == "exit")
					break;
				
				
				
			}
		}
	}
}