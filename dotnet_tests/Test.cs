using System;
using System.Linq;
using System.Collections.Generic;
using Xunit;
using etebase_csharp;

namespace dotnet
{
    public class UnitTest1
    {
        [Fact]
        public void Test1()
        {
            var bytes = new byte[] { 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20 };
            var b64 = etebase_csharp.Utils.ToBase64(bytes);
            var bytes2 = etebase_csharp.Utils.FromBase64(b64);
            Assert.True(bytes.SequenceEqual(bytes));
        }
    }
}

